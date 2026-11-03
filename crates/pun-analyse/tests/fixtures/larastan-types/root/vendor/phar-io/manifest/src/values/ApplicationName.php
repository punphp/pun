<?php declare(strict_types = 1);
/*
 * This file is part of PharIo\Manifest.
 *
 * Copyright (c) Arne Blankerts <arne@blankerts.de>, Sebastian Heuer <sebastian@phpeople.de>, Sebastian Bergmann <sebastian@phpunit.de> and contributors
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 */
namespace PharIo\Manifest;

use function preg_match;
use function sprintf;

class ApplicationName {
    /** @var string */
    private $name;

    public function __construct(string $name) {}

    public function asString(): string {}

    public function isEqual(ApplicationName $name): bool {}

    private function ensureValidFormat(string $name): void {}
}
