<?php

declare(strict_types=1);

namespace League\Flysystem\Local;

use League\MimeTypeDetection\MimeTypeDetector;
use function in_array;

class FallbackMimeTypeDetector implements MimeTypeDetector
{
    private const INCONCLUSIVE_MIME_TYPES = [
        'application/x-empty',
        'text/plain',
        'text/x-asm',
        'application/octet-stream',
        'inode/x-empty',
    ];

    public function __construct(
        private MimeTypeDetector $detector,
        private array $inconclusiveMimetypes = self::INCONCLUSIVE_MIME_TYPES,
        private bool $useInconclusiveMimeTypeFallback = false,
    ) {}

    public function detectMimeType(string $path, $contents): ?string
    {}

    public function detectMimeTypeFromBuffer(string $contents): ?string
    {}

    public function detectMimeTypeFromPath(string $path): ?string
    {}

    public function detectMimeTypeFromFile(string $path): ?string
    {}
}
