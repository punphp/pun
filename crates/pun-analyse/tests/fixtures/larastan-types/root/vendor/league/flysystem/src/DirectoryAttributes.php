<?php

declare(strict_types=1);

namespace League\Flysystem;

class DirectoryAttributes implements StorageAttributes
{
    use ProxyArrayAccessToProperties;
    private string $type = StorageAttributes::TYPE_DIRECTORY;

    public function __construct(
        private string $path,
        private ?string $visibility = null,
        private ?int $lastModified = null,
        private array $extraMetadata = []
    ) {}

    public function path(): string
    {}

    public function type(): string
    {}

    public function visibility(): ?string
    {}

    public function lastModified(): ?int
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

    /**
     * @inheritDoc
     */
    public function jsonSerialize(): array
    {}
}
