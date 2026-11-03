<?php declare(strict_types = 1);
namespace PharIo\Version;

class VersionConstraintValue {
    /** @var VersionNumber */
    private $major;

    /** @var VersionNumber */
    private $minor;

    /** @var VersionNumber */
    private $patch;

    /** @var string */
    private $label = '';

    /** @var string */
    private $buildMetaData = '';

    /** @var string */
    private $versionString = '';

    public function __construct(string $versionString) {}

    public function getLabel(): string {}

    public function getBuildMetaData(): string {}

    public function getVersionString(): string {}

    public function getMajor(): VersionNumber {}

    public function getMinor(): VersionNumber {}

    public function getPatch(): VersionNumber {}

    private function parseVersion(string $versionString): void {}

    private function extractBuildMetaData(string &$versionString): void {}

    private function extractLabel(string &$versionString): void {}

    private function stripPotentialVPrefix(string &$versionString): void {}
}
