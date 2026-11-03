<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation;

use Symfony\Component\HttpFoundation\File\Exception\FileException;
use Symfony\Component\HttpFoundation\File\Exception\FileNotFoundException;
use Symfony\Component\HttpFoundation\File\File;

/**
 * BinaryFileResponse represents an HTTP response delivering a file.
 *
 * @author Niklas Fiekas <niklas.fiekas@tu-clausthal.de>
 * @author stealth35 <stealth35-php@live.fr>
 * @author Igor Wiedler <igor@wiedler.ch>
 * @author Jordan Alliot <jordan.alliot@gmail.com>
 * @author Sergey Linnik <linniksa@gmail.com>
 */
class BinaryFileResponse extends Response
{
    // Values accepted for the "X-Sendfile-Type" header, mapped to the response header to send
    private const SENDFILE_TYPES = [
        'x-sendfile' => 'X-Sendfile',
        'x-accel-redirect' => 'X-Accel-Redirect',
        'x-lighttpd-send-file' => 'X-LIGHTTPD-send-file',
    ];

    protected static bool $trustXSendfileTypeHeader = false;

    protected File $file;
    protected ?\SplTempFileObject $tempFileObject = null;
    protected int $offset = 0;
    protected int $maxlen = -1;
    protected bool $deleteFileAfterSend = false;
    protected int $chunkSize = 16 * 1024;

    /**
     * @param \SplFileInfo|string $file               The file to stream
     * @param int                 $status             The response status code (200 "OK" by default)
     * @param array               $headers            An array of response headers
     * @param bool                $public             Files are public by default
     * @param string|null         $contentDisposition The type of Content-Disposition to set automatically with the filename
     * @param bool                $autoEtag           Whether the ETag header should be automatically set
     * @param bool                $autoLastModified   Whether the Last-Modified header should be automatically set
     *
     * @throws FileNotFoundException If the given path is not a file
     * @throws FileException         If the file is not readable
     */
    public function __construct(\SplFileInfo|string $file, int $status = 200, array $headers = [], bool $public = true, ?string $contentDisposition = null, bool $autoEtag = false, bool $autoLastModified = true)
    {}

    /**
     * Sets the file to stream.
     *
     * @return $this
     *
     * @throws FileNotFoundException If the given path is not a file
     * @throws FileException         If the file is not readable
     */
    public function setFile(\SplFileInfo|string $file, ?string $contentDisposition = null, bool $autoEtag = false, bool $autoLastModified = true): static
    {}

    /**
     * Gets the file.
     */
    public function getFile(): File
    {}

    /**
     * Sets the response stream chunk size.
     *
     * @return $this
     */
    public function setChunkSize(int $chunkSize): static
    {}

    /**
     * Automatically sets the Last-Modified header according the file modification date.
     *
     * @return $this
     */
    public function setAutoLastModified(): static
    {}

    /**
     * Automatically sets the ETag header according to the checksum of the file.
     *
     * @return $this
     */
    public function setAutoEtag(): static
    {}

    /**
     * Sets the Content-Disposition header with the given filename.
     *
     * @param string $disposition      ResponseHeaderBag::DISPOSITION_INLINE or ResponseHeaderBag::DISPOSITION_ATTACHMENT
     * @param string $filename         Optionally use this UTF-8 encoded filename instead of the real name of the file
     * @param string $filenameFallback A fallback filename, containing only ASCII characters. Defaults to an automatically encoded filename
     *
     * @return $this
     */
    public function setContentDisposition(string $disposition, string $filename = '', string $filenameFallback = ''): static
    {}

    public function prepare(Request $request): static
    {}

    private function hasValidIfRangeHeader(?string $header): bool
    {}

    public function sendContent(): static
    {}

    /**
     * @throws \LogicException when the content is not null
     */
    public function setContent(?string $content): static
    {}

    public function getContent(): string|false
    {}

    /**
     * Trust X-Sendfile-Type header.
     */
    public static function trustXSendfileTypeHeader(): void
    {}

    /**
     * If this is set to true, the file will be unlinked after the request is sent
     * Note: If the X-Sendfile header is used, the deleteFileAfterSend setting will not be used.
     *
     * @return $this
     */
    public function deleteFileAfterSend(bool $shouldDelete = true): static
    {}

    /**
     * Returns whether the file will be unlinked after the request is sent.
     */
    public function shouldDeleteFileAfterSend(): bool
    {}
}
