<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts pgsql resources to array representation.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class PgSqlCaster
{
    private const PARAM_CODES = [
        'server_encoding',
        'client_encoding',
        'is_superuser',
        'session_authorization',
        'DateStyle',
        'TimeZone',
        'IntervalStyle',
        'integer_datetimes',
        'application_name',
        'standard_conforming_strings',
    ];

    private const TRANSACTION_STATUS = [
        \PGSQL_TRANSACTION_IDLE => 'PGSQL_TRANSACTION_IDLE',
        \PGSQL_TRANSACTION_ACTIVE => 'PGSQL_TRANSACTION_ACTIVE',
        \PGSQL_TRANSACTION_INTRANS => 'PGSQL_TRANSACTION_INTRANS',
        \PGSQL_TRANSACTION_INERROR => 'PGSQL_TRANSACTION_INERROR',
        \PGSQL_TRANSACTION_UNKNOWN => 'PGSQL_TRANSACTION_UNKNOWN',
    ];

    private const RESULT_STATUS = [
        \PGSQL_EMPTY_QUERY => 'PGSQL_EMPTY_QUERY',
        \PGSQL_COMMAND_OK => 'PGSQL_COMMAND_OK',
        \PGSQL_TUPLES_OK => 'PGSQL_TUPLES_OK',
        \PGSQL_COPY_OUT => 'PGSQL_COPY_OUT',
        \PGSQL_COPY_IN => 'PGSQL_COPY_IN',
        \PGSQL_BAD_RESPONSE => 'PGSQL_BAD_RESPONSE',
        \PGSQL_NONFATAL_ERROR => 'PGSQL_NONFATAL_ERROR',
        \PGSQL_FATAL_ERROR => 'PGSQL_FATAL_ERROR',
    ];

    private const DIAG_CODES = [
        'severity' => \PGSQL_DIAG_SEVERITY,
        'sqlstate' => \PGSQL_DIAG_SQLSTATE,
        'message' => \PGSQL_DIAG_MESSAGE_PRIMARY,
        'detail' => \PGSQL_DIAG_MESSAGE_DETAIL,
        'hint' => \PGSQL_DIAG_MESSAGE_HINT,
        'statement position' => \PGSQL_DIAG_STATEMENT_POSITION,
        'internal position' => \PGSQL_DIAG_INTERNAL_POSITION,
        'internal query' => \PGSQL_DIAG_INTERNAL_QUERY,
        'context' => \PGSQL_DIAG_CONTEXT,
        'file' => \PGSQL_DIAG_SOURCE_FILE,
        'line' => \PGSQL_DIAG_SOURCE_LINE,
        'function' => \PGSQL_DIAG_SOURCE_FUNCTION,
    ];

    public static function castLargeObject($lo, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castLink($link, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castResult($result, array $a, Stub $stub, bool $isNested): array
    {}
}
