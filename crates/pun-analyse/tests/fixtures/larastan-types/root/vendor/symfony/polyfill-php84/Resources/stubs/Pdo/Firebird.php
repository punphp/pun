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

if (\PHP_VERSION_ID < 80400 && \extension_loaded('pdo_firebird')) {
    class Firebird extends \PDO
    {
        public const ATTR_DATE_FORMAT = \PDO::FB_ATTR_DATE_FORMAT;
        public const ATTR_TIME_FORMAT = \PDO::FB_ATTR_TIME_FORMAT;
        public const ATTR_TIMESTAMP_FORMAT = \PDO::FB_ATTR_TIMESTAMP_FORMAT;

        public function __construct(string $dsn, ?string $username = null, ?string $password = null, ?array $options = null)
        {}

        public static function connect(string $dsn, ?string $username = null, ?string $password = null, ?array $options = null): self
        {}
    }
}
