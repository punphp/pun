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

class Manifest {
    /** @var ApplicationName */
    private $name;

    /** @var Version */
    private $version;

    /** @var Type */
    private $type;

    /** @var CopyrightInformation */
    private $copyrightInformation;

    /** @var RequirementCollection */
    private $requirements;

    /** @var BundledComponentCollection */
    private $bundledComponents;

    public function __construct(ApplicationName $name, Version $version, Type $type, CopyrightInformation $copyrightInformation, RequirementCollection $requirements, BundledComponentCollection $bundledComponents) {}

    public function getName(): ApplicationName {}

    public function getVersion(): Version {}

    public function getType(): Type {}

    public function getCopyrightInformation(): CopyrightInformation {}

    public function getRequirements(): RequirementCollection {}

    public function getBundledComponents(): BundledComponentCollection {}

    public function isApplication(): bool {}

    public function isLibrary(): bool {}

    public function isExtension(): bool {}

    public function isExtensionFor(ApplicationName $application, ?Version $version = null): bool {}
}
