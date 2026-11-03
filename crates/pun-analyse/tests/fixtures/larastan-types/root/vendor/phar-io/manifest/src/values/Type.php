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

use PharIo\Version\VersionConstraint;

abstract class Type {
    public static function application(): Application {}

    public static function library(): Library {}

    public static function extension(ApplicationName $application, VersionConstraint $versionConstraint): Extension {}

    /** @psalm-assert-if-true Application $this */
    public function isApplication(): bool {}

    /** @psalm-assert-if-true Library $this */
    public function isLibrary(): bool {}

    /** @psalm-assert-if-true Extension $this */
    public function isExtension(): bool {}
}
