<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

if (\PHP_VERSION_ID < 80400) {
    // @author Daniel Scherzer <daniel.e.scherzer@gmail.com>
    final class ReflectionConstant
    {
        /**
         * @var string
         *
         * @readonly
         */
        public $name;

        private $value;
        private $deprecated;
        private $persistent;

        private static $persistentConstants = [];

        public function __construct(string $name)
        {}

        public function getName(): string
        {}

        public function getValue()
        {}

        public function getNamespaceName(): string
        {}

        public function getShortName(): string
        {}

        public function isDeprecated(): bool
        {}

        public function __toString(): string
        {}

        public function __sleep(): array
        {}

        public function __wakeup(): void
        {}
    }
}
