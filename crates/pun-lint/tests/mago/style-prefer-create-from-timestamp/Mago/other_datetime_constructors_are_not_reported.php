<?php

$timestamp = time();

$already_direct = DateTimeImmutable::createFromTimestamp($timestamp);
$non_timestamp = new DateTimeImmutable($timestamp);
$with_timezone = new DateTimeImmutable('@' . $timestamp, new DateTimeZone('UTC'));
$mutable = new DateTime('@' . $timestamp);
$not_a_timestamp = new DateTimeImmutable('timestamp: ' . $timestamp);
