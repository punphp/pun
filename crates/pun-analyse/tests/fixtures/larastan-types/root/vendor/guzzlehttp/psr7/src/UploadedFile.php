<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

use InvalidArgumentException;
use Psr\Http\Message\StreamInterface;
use Psr\Http\Message\UploadedFileInterface;
use RuntimeException;

class UploadedFile implements UploadedFileInterface
{
    private const ERROR_MAP = [
        UPLOAD_ERR_OK => 'UPLOAD_ERR_OK',
        UPLOAD_ERR_INI_SIZE => 'UPLOAD_ERR_INI_SIZE',
        UPLOAD_ERR_FORM_SIZE => 'UPLOAD_ERR_FORM_SIZE',
        UPLOAD_ERR_PARTIAL => 'UPLOAD_ERR_PARTIAL',
        UPLOAD_ERR_NO_FILE => 'UPLOAD_ERR_NO_FILE',
        UPLOAD_ERR_NO_TMP_DIR => 'UPLOAD_ERR_NO_TMP_DIR',
        UPLOAD_ERR_CANT_WRITE => 'UPLOAD_ERR_CANT_WRITE',
        UPLOAD_ERR_EXTENSION => 'UPLOAD_ERR_EXTENSION',
    ];

    private ?string $clientFilename;

    private ?string $clientMediaType;

    private int $error;

    private ?string $file = null;

    private bool $moved = false;

    private ?int $size;

    private ?StreamInterface $stream = null;

    /**
     * @param StreamInterface|string|resource $streamOrFile
     */
    public function __construct(
        $streamOrFile,
        ?int $size,
        int $errorStatus,
        ?string $clientFilename = null,
        ?string $clientMediaType = null
    ) {}

    /**
     * Depending on the value set file or stream variable
     *
     * @param StreamInterface|string|resource $streamOrFile
     *
     * @throws InvalidArgumentException
     */
    private function setStreamOrFile($streamOrFile): void
    {}

    /**
     * @throws InvalidArgumentException
     */
    private function setError(int $error): void
    {}

    /**
     * Return true if there is no upload error
     */
    private function isOk(): bool
    {}

    public function isMoved(): bool
    {}

    /**
     * @throws RuntimeException if is moved or not ok
     */
    private function validateActive(): void
    {}

    public function getStream(): StreamInterface
    {}

    public function moveTo(string $targetPath): void
    {}

    public function getSize(): ?int
    {}

    public function getError(): int
    {}

    public function getClientFilename(): ?string
    {}

    public function getClientMediaType(): ?string
    {}
}
