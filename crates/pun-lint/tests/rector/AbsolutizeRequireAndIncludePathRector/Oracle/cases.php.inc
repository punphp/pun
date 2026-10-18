<?php

require('a.php');
require_once 'a/' . 'b.php';
require '/abs/' . $x;
require './a/' . $x;
require 'phar://a/' . $x;
include 'C:\\x\\y.php';
include 'c:/x.php';
include '_:/x.php';
include 'a\\b.php';
include "a\\b\$c.php";
include 'it\'s.php';
include "tab\tx.php";
include '../config/x.php';
include $x . 'a.php';
include ('x') . $y;
-----
<?php

require(__DIR__ . '/a.php');
require_once __DIR__ . '/a/' . 'b.php';
require __DIR__ . '/abs/' . $x;
require __DIR__ . '/a/' . $x;
require 'phar://a/' . $x;
include 'C:\\x\\y.php';
include 'c:/x.php';
include '_:/x.php';
include __DIR__ . '/a\b.php';
include __DIR__ . "/a\\b\$c.php";
include __DIR__ . '/it\'s.php';
include __DIR__ . "/tab\tx.php";
include '../config/x.php';
include $x . 'a.php';
include (__DIR__ . '/x') . $y;
