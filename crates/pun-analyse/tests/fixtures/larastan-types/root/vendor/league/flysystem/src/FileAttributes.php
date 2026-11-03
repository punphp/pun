<?php

declare(strict_types=1);

namespace League\Flysystem;

class FileAttributes implements StorageAttributes
{
    use ProxyArrayAccessToProperties;
    private string $type = StorageAttributes::TYPE_FILE;

    public function __construct(
        private string $path,
        private ?int $fileSize = null,
        private ?string $visibility = null,
        private ?int $lastModified = null,
        private ?string $mimeType = null,
        private array $extraMetadata = []
    ) {}

    public function type(): string
    {}

    public function path(): string
    {}

    public function fileSize(): ?int
    {}

    public function visibility(): ?string
    {}

    public function lastModified(): ?int
    {}

    public function mimeType(): ?string
    {}

    public function extraMetadata(): array
    {}

    public function isFile(): bool
    {}

    public function isDir(): bool
    {}

    public function withPath(string $path): self
    {}

    public static function fromArray(array $attributes): self
    {}

    public function jsonSerialize(): array
    {}
}
