// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/native_assets_cli.dart';
import 'dart:io';

const crateName = 'diplomat-example';
const assetId = 'package:example/src/lib.g.dart';

void main(List<String> args) async {
  final config = await BuildConfig.fromArgs(args);

  final cargo = await Process.run('cargo', [
    'rustc',
    '-p',
    crateName,
    '--crate-type=cdylib',
  ]);

  if (cargo.exitCode != 0) {
    throw cargo.stderr;
  }

  final libPath =
      '../../target/debug/${Target.current.os.dylibFileName(crateName.replaceAll('-', '_'))}';

  await File(libPath).copy('${config.outDir.path}/lib');

  await BuildOutput(
    assets: [
      Asset(
          id: assetId,
          linkMode: LinkMode.static,
          target: Target.current,
          path: AssetAbsolutePath(Uri.file('${config.outDir.path}/lib')))
    ],
    dependencies: Dependencies([Uri.file('build.dart'), Uri.file(libPath)]),
  ).writeToFile(outDir: config.outDir);
}
