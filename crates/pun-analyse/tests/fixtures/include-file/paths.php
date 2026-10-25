<?php
include 'cwd.txt';
include 'include.txt';
include 'script.txt';
include './cwd.txt';
include './include.txt';
include '../cwd/cwd.txt';
include 'nested/../script.txt';
include '/not-a-real-pun-include-path/missing.php';
include 'file://' . __DIR__ . '/script.txt';
include 'file://localhost' . __DIR__ . '/script.txt';
