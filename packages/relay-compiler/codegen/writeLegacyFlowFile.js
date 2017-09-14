/**
 * Copyright (c) 2013-present, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the BSD-style license found in the
 * LICENSE file in the root directory of this source tree. An additional grant
 * of patent rights can be found in the PATENTS file in the same directory.
 *
 * @providesModule writeLegacyFlowFile
 * @flow
 * @format
 */

'use strict';

const SignedSource = require('signedsource');

import type {CodegenDirectory} from '../graphql-compiler/GraphQLCompilerPublic';

function writeLegacyFlowFile(
  outputDirectory: CodegenDirectory,
  name: string,
  flowTypes: string,
  platform?: ?string,
): void {
  const moduleName = name + '.legacyflow';
  const header = `/**
 * Copyright 2004-present Facebook. All Rights Reserved.
 *
 * ${'@'}providesModule ${moduleName}
 * ${SignedSource.getSigningToken()}
 * ${'@'}flow
 */

'use strict';

/**
 * NOTE:
 * These are legacy flow types that have issues in some edge cases. For example:
 *
 *   fragment on Actor {
 *     ... on User {
 *       name
 *     }
 *     ... on Page {
 *       name
 *     }
 *   }
 *
 * generates invalid output. Please use the flow types from the *.graphql.js
 * file instead.
 */

`;

  const fileName = platform ? moduleName + '.' + platform : moduleName;
  outputDirectory.writeFile(
    fileName + '.js',
    SignedSource.signFile(header + flowTypes + '\n'),
  );
}

module.exports = writeLegacyFlowFile;
