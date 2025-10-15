// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:io';

import 'package:code_assets/code_assets.dart';
import 'package:hooks/hooks.dart';

const crateName = 'diplomat-feature-tests';

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

    output.assets.code.add(
      CodeAsset(
        package: input.packageName,
        name: 'src/lib.g.dart',
        linkMode: DynamicLoadingBundled(),
        file: input.packageRoot.resolve(
          '../../target/debug/${input.config.code.targetOS.dylibFileName(crateName.replaceAll('-', '_'))}',
        ),
      ),
    );

    output.dependencies.add(input.packageRoot.resolve('build.rs'));
  });
}
