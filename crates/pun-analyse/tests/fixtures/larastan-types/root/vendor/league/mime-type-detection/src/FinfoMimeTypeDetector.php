<?php

declare(strict_types=1);

namespace League\MimeTypeDetection;

use const FILEINFO_MIME_TYPE;

use const PATHINFO_EXTENSION;
use finfo;

class FinfoMimeTypeDetector implements MimeTypeDetector, ExtensionLookup
{
    private const INCONCLUSIVE_MIME_TYPES = [
        'application/x-empty',
        'text/plain',
        'text/x-asm',
        'application/octet-stream',
        'inode/x-empty',
    ];

    /**
     * @var finfo
     */
    private $finfo;

    /**
     * @var ExtensionToMimeTypeMap
     */
    private $extensionMap;

    /**
     * @var int|null
     */
    private $bufferSampleSize;

    /**
     * @var array<string>
     */
    private $inconclusiveMimetypes;

    public function __construct(
        string $magicFile = '',
        ?ExtensionToMimeTypeMap $extensionMap = null,
        ?int $bufferSampleSize = null,
        array $inconclusiveMimetypes = self::INCONCLUSIVE_MIME_TYPES
    ) {}

    public function detectMimeType(string $path, $contents): ?string
    {}

    public function detectMimeTypeFromPath(string $path): ?string
    {}

    public function detectMimeTypeFromFile(string $path): ?string
    {}

    public function detectMimeTypeFromBuffer(string $contents): ?string
    {}

    private function takeSample(string $contents): string
    {}

    public function lookupExtension(string $mimetype): ?string
    {}

    public function lookupAllExtensions(string $mimetype): array
    {}
}
