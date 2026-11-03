<?php

declare(strict_types=1);

namespace League\Flysystem\UnixVisibility;

use League\Flysystem\PortableVisibilityGuard;
use League\Flysystem\Visibility;

class PortableVisibilityConverter implements VisibilityConverter
{
    public function __construct(
        private int $filePublic = 0644,
        private int $filePrivate = 0600,
        private int $directoryPublic = 0755,
        private int $directoryPrivate = 0700,
        private string $defaultForDirectories = Visibility::PRIVATE
    ) {}

    public function forFile(string $visibility): int
    {}

    public function forDirectory(string $visibility): int
    {}

    public function inverseForFile(int $visibility): string
    {}

    public function inverseForDirectory(int $visibility): string
    {}

    public function defaultForDirectories(): int
    {}

    /**
     * @param array<mixed>  $permissionMap
     */
    public static function fromArray(array $permissionMap, string $defaultForDirectories = Visibility::PRIVATE): PortableVisibilityConverter
    {}
}
