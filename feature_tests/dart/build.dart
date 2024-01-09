// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/native_assets_cli.dart';
import 'dart:io';

const crateName = 'diplomat-feature-tests';
const release = false;
const assetId = 'package:feature_tests/src/lib.g.dart';

void main(List<String> args) async {
  final config = await BuildConfig.fromArgs(args);

  await Process.run('cargo', [
    'rustc',
    '-p',
    crateName,
    '--crate-type=cdylib',
    if (release) '--release'
  ], environment: {
    'CARGO_TARGET_DIR': config.outDir.path
  });

  final libPath =
      '${config.outDir.path}/${release ? 'release' : 'debug'}/lib${crateName.replaceAll("-", "_")}.${Platform.isMacOS ? 'dylib' : Platform.isWindows ? 'dll' : 'so'}';

  await BuildOutput(
    assets: [
      Asset(
          id: assetId,
          linkMode: LinkMode.static,
          target: Target.current,
          path: AssetAbsolutePath(Uri.file(libPath)))
    ],
    dependencies: Dependencies([Uri.file('build.dart')]),
  ).writeToFile(outDir: config.outDir);
}
