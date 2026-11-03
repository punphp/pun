<?php declare(strict_types = 1);
/*
 * This file is part of PharIo\Version.
 *
 * (c) Arne Blankerts <arne@blankerts.de>, Sebastian Heuer <sebastian@phpeople.de>, Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PharIo\Version;

class Version {
    /** @var string */
    private $originalVersionString;

    /** @var VersionNumber */
    private $major;

    /** @var VersionNumber */
    private $minor;

    /** @var VersionNumber */
    private $patch;

    /** @var null|PreReleaseSuffix */
    private $preReleaseSuffix;

    /** @var null|BuildMetaData */
    private $buildMetadata;

    public function __construct(string $versionString) {}

    /**
     * @throws NoPreReleaseSuffixException
     */
    public function getPreReleaseSuffix(): PreReleaseSuffix {}

    public function getOriginalString(): string {}

    public function getVersionString(): string {}

    public function hasPreReleaseSuffix(): bool {}

    public function equals(Version $other): bool {}

    public function isGreaterThan(Version $version): bool {}

    public function getMajor(): VersionNumber {}

    public function getMinor(): VersionNumber {}

    public function getPatch(): VersionNumber {}

    /**
     * @psalm-assert-if-true BuildMetaData $this->buildMetadata
     * @psalm-assert-if-true BuildMetaData $this->getBuildMetaData()
     */
    public function hasBuildMetaData(): bool {}

    /**
     * @throws NoBuildMetaDataException
     */
    public function getBuildMetaData(): BuildMetaData {}

    /**
     * @param string[] $matches
     *
     * @throws InvalidPreReleaseSuffixException
     */
    private function parseVersion(array $matches): void {}

    /**
     * @param string $version
     *
     * @throws InvalidVersionException
     */
    private function ensureVersionStringIsValid($version): void {}
}
