<?php
namespace ClassCaseOptions\Internal;

#[\nodiscard]
function lowerCaseAttribute(): int { return 1; }

#[\ATTRIBUTE]
class StubbedAttribute {}

new \datetimeimmutable();
\datetimeimmutable::createFromFormat('Y', '2020');
\datetimeinterface::ATOM;
function signature(\datetimeimmutable $value): \datetimeimmutable { return $value; }

use DateTimeImmutable;
new datetimeimmutable();

use DateTimeImmutable as Clock;
new Clock();
new clock();
