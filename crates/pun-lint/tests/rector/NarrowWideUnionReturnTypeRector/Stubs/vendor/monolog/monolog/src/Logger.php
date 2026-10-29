<?php

// monolog/monolog 3's Logger and LogRecord, as the vendor code the fixture names: a constant whose value differs
// between the package's major versions.

namespace Monolog;

class Logger
{
    public const API = 3;
}

class LogRecord
{
}
