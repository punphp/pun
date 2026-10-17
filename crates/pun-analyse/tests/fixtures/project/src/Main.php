<?php

namespace Main;

use App\Lib;
use App\Dev\DevTool;
use Old_Thing;
use Mapped;
use Single;
use Acme\Pkg\Tool;
use Acme_Str;
use Mapped_In_Package;
use ScanOnly\Helper;
use Excluded\Gone;

new Lib();
new DevTool();
new Old_Thing();
new Mapped();
new Single();
new Tool();
new Acme_Str();
new Mapped_In_Package();
new Helper();
new Gone();
new FromScanDir();
\from_scan_file();
\helper_function();
\acme_helper();
\ignored_missing();
\really_missing();
