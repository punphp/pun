<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Pdo;

if (\PHP_VERSION_ID < 80400 && \extension_loaded('pdo_odbc')) {
    class Odbc extends \PDO
    {
        public const ATTR_USE_CURSOR_LIBRARY = \PDO::ODBC_ATTR_USE_CURSOR_LIBRARY;
        public const ATTR_ASSUME_UTF8 = \PDO::ODBC_ATTR_ASSUME_UTF8;
        public const SQL_USE_IF_NEEDED = \PDO::ODBC_SQL_USE_IF_NEEDED;
        public const SQL_USE_DRIVER = \PDO::ODBC_SQL_USE_DRIVER;
        public const SQL_USE_ODBC = \PDO::ODBC_SQL_USE_ODBC;

        public function __construct(string $dsn, ?string $username = null, ?string $password = null, ?array $options = null)
        {}

        public static function connect(string $dsn, ?string $username = null, ?string $password = null, ?array $options = null): self
        {}
    }
}
