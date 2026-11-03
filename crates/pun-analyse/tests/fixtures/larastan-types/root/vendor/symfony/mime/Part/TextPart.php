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

use Symfony\Component\Mime\Encoder\Base64ContentEncoder;
use Symfony\Component\Mime\Encoder\ContentEncoderInterface;
use Symfony\Component\Mime\Encoder\EightBitContentEncoder;
use Symfony\Component\Mime\Encoder\QpContentEncoder;
use Symfony\Component\Mime\Exception\InvalidArgumentException;
use Symfony\Component\Mime\Header\Headers;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class TextPart extends AbstractPart
{
    private const DEFAULT_ENCODERS = ['quoted-printable', 'base64', '8bit'];

    private static array $encoders = [];

    /** @var resource|string|File */
    private $body;
    private ?string $charset;
    private string $subtype;
    private ?string $disposition = null;
    private ?string $name = null;
    private string $encoding;
    private ?bool $seekable = null;

    /**
     * @param resource|string|File $body Use a File instance to defer loading the file until rendering
     */
    public function __construct($body, ?string $charset = 'utf-8', string $subtype = 'plain', ?string $encoding = null)
    {}

    public function getMediaType(): string
    {}

    public function getMediaSubtype(): string
    {}

    /**
     * @param string $disposition one of attachment, inline, or form-data
     *
     * @return $this
     */
    public function setDisposition(string $disposition): static
    {}

    /**
     * @return ?string null or one of attachment, inline, or form-data
     */
    public function getDisposition(): ?string
    {}

    /**
     * Sets the name of the file (used by FormDataPart).
     *
     * @return $this
     */
    public function setName(string $name): static
    {}

    /**
     * Gets the name of the file.
     */
    public function getName(): ?string
    {}

    public function getBody(): string
    {}

    public function bodyToString(): string
    {}

    public function bodyToIterable(): iterable
    { yield; }

    public function getPreparedHeaders(): Headers
    {}

    public function asDebugString(): string
    {}

    private function getEncoder(): ContentEncoderInterface
    {}

    public static function addEncoder(ContentEncoderInterface $encoder): void
    {}

    private function chooseEncoding(): string
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}
}
