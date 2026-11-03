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

use PharIo\Version\AnyVersionConstraint;
use PharIo\Version\Version;
use PharIo\Version\VersionConstraint;
use XMLWriter;
use function count;
use function file_put_contents;
use function str_repeat;

/** @psalm-suppress MissingConstructor */
class ManifestSerializer {
    /** @var XMLWriter */
    private $xmlWriter;

    public function serializeToFile(Manifest $manifest, string $filename): void {}

    public function serializeToString(Manifest $manifest): string {}

    private function startDocument(): void {}

    private function finishDocument(): string {}

    private function addContains(ApplicationName $name, Version $version, Type $type): void {}

    private function addCopyright(CopyrightInformation $copyrightInformation): void {}

    private function addRequirements(RequirementCollection $requirementCollection): void {}

    private function addBundles(BundledComponentCollection $bundledComponentCollection): void {}

    private function addExtension(ApplicationName $applicationName, VersionConstraint $versionConstraint): void {}
}
