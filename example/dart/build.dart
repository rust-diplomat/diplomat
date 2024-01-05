// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/native_assets_cli.dart';
import 'dart:io';

void main(List<String> args) async {
  final config = await BuildConfig.fromArgs(args);

  await Process.run(
      'cargo', ['rustc', '-p', 'diplomat-example', '--crate-type=cdylib'],
      environment: {'CARGO_TARGET_DIR': config.outDir.path});

  await BuildOutput(
    assets: [
      Asset(
          id: 'package:example/src/lib.g.dart',
          linkMode: LinkMode.static,
          target: Target.current,
          path: AssetAbsolutePath(Uri.file(
              '${config.outDir.path}/debug/libdiplomat_example.${Platform.isMacOS ? 'dylib' : Platform.isWindows ? 'dll' : 'so'}')))
    ],
    dependencies: Dependencies([Uri.file('build.dart')]),
  ).writeToFile(outDir: config.outDir);
}
