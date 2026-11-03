<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime\Part;

use Symfony\Component\Mime\Exception\InvalidArgumentException;
use Symfony\Component\Mime\Header\Headers;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class DataPart extends TextPart
{
    private ?string $filename = null;
    private string $mediaType;
    private ?string $cid = null;

    /**
     * @param resource|string|File $body Use a File instance to defer loading the file until rendering
     */
    public function __construct($body, ?string $filename = null, ?string $contentType = null, ?string $encoding = null)
    {}

    public static function fromPath(string $path, ?string $name = null, ?string $contentType = null): self
    {}

    /**
     * @return $this
     */
    public function asInline(): static
    {}

    /**
     * @return $this
     */
    public function setContentId(string $cid): static
    {}

    public function getContentId(): string
    {}

    public function hasContentId(): bool
    {}

    public function getMediaType(): string
    {}

    public function getPreparedHeaders(): Headers
    {}

    public function asDebugString(): string
    {}

    public function getFilename(): ?string
    {}

    public function getContentType(): string
    {}

    private function generateContentId(): string
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}
}
