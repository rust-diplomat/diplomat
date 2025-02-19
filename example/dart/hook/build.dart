// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/code_assets.dart';
import 'dart:io';

const crateName = 'diplomat-example';

void main(List<String> args) async {
  await build(args, (input, output) async {
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
        '../../target/debug/${input.config.code.targetOS.dylibFileName(crateName.replaceAll('-', '_'))}';

    output.assets.code.add(
      CodeAsset(
        package: input.packageName,
        name: 'src/lib.g.dart',
        linkMode: DynamicLoadingBundled(),
        os: input.config.code.targetOS,
        architecture: input.config.code.targetArchitecture,
        file: Uri.file(libPath),
      ),
    );

    output.addDependency(input.packageRoot.resolve('build.rs'));
  });
}
