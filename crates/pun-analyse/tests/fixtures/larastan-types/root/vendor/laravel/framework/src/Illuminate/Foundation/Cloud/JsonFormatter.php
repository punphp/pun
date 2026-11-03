<?php

namespace Illuminate\Foundation\Cloud;

use Illuminate\Container\Container;
use Monolog\Formatter\JsonFormatter as BaseFormatter;
use Monolog\LogRecord;

class JsonFormatter extends BaseFormatter
{
    /**
     * {@inheritdoc}
     */
    protected function normalizeRecord(LogRecord $record): array
    {}
}
