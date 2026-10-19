<?php

namespace ImportUses;

use const ImportDefs\DEFINED;
use const ImportDefs\RUNTIME;
use const ImportDefs\MISSING;
use const GLOBAL_RUNTIME;
use const PHP_VERSION;
use const php_version;
use const E_ALL, \E_NOPE;
use const importdefs\DEFINED as Lower;
use function ImportDefs\helper, ImportDefs\nope;
use ImportDefs\{const DEFINED as D2, const GONE, function helper as h2, function gone};
use const ImportDefs\{DEFINED as D3, ABSENT};
use function ImportDefs\{helper as h3, absent};
