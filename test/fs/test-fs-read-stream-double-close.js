// Copyright Joyent and Node contributors. All rights reserved. MIT license.
'use strict';

import common from '../common';
import fs from 'fs';

{
  const s = fs.createReadStream(__filename);

  s.close(common.mustCall());
  s.close(common.mustCall());
}

{
  const s = fs.createReadStream(__filename);

  // This is a private API, but it is worth testing. close calls this
  s.destroy(null, common.mustCall());
  s.destroy(null, common.mustCall());
}
