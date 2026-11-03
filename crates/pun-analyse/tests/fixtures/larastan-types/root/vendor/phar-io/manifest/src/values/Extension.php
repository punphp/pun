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

use PharIo\Version\Version;
use PharIo\Version\VersionConstraint;

class Extension extends Type {
    /** @var ApplicationName */
    private $application;

    /** @var VersionConstraint */
    private $versionConstraint;

    public function __construct(ApplicationName $application, VersionConstraint $versionConstraint) {}

    public function getApplicationName(): ApplicationName {}

    public function getVersionConstraint(): VersionConstraint {}

    public function isExtension(): bool {}

    public function isExtensionFor(ApplicationName $name): bool {}

    public function isCompatibleWith(ApplicationName $name, Version $version): bool {}
}
