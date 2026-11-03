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

use PharIo\Version\Exception as VersionException;
use PharIo\Version\Version;
use PharIo\Version\VersionConstraintParser;
use Throwable;
use function sprintf;

class ManifestDocumentMapper {
    public function map(ManifestDocument $document): Manifest {}

    private function mapType(ContainsElement $contains): Type {}

    private function mapCopyright(CopyrightElement $copyright): CopyrightInformation {}

    private function mapRequirements(RequiresElement $requires): RequirementCollection {}

    private function mapBundledComponents(ManifestDocument $document): BundledComponentCollection {}

    private function mapExtension(ExtensionElement $extension): Extension {}
}
