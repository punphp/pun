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

if (\PHP_VERSION_ID < 80400 && \extension_loaded('pdo_pgsql')) {
    class Pgsql extends \PDO
    {
        public const ATTR_DISABLE_PREPARES = \PDO::PGSQL_ATTR_DISABLE_PREPARES;

        public function __construct(string $dsn, ?string $username = null, ?string $password = null, ?array $options = null)
        {}

        public static function connect(string $dsn, ?string $username = null, ?string $password = null, ?array $options = null): self
        {}

        public function copyFromArray(string $tableName, array $rows, string $separator = "\t", string $nullAs = '\\\\N', ?string $fields = null): bool
        {}

        public function copyFromFile(string $tableName, string $filename, string $separator = "\t", string $nullAs = '\\\\N', ?string $fields = null): bool
        {}

        /**
         * @return array|false
         */
        public function copyToArray(string $tableName, string $separator = "\t", string $nullAs = '\\\\N', ?string $fields = null)
        {}

        public function copyToFile(string $tableName, string $filename, string $separator = "\t", string $nullAs = '\\\\N', ?string $fields = null): bool
        {}

        /**
         * @return array|false
         */
        public function getNotify(int $fetchMode = \PDO::FETCH_DEFAULT, int $timeoutMilliseconds = 0)
        {}

        public function getPid(): int
        {}

        /**
         * @return string|false
         */
        public function lobCreate()
        {}

        /**
         * @return resource|false
         */
        public function lobOpen(string $oid, string $mode = 'rb')
        {}

        public function lobUnlink(string $oid): bool
        {}
    }
}
