<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime\Header;

use Symfony\Component\Mime\Encoder\Rfc2231Encoder;
use Symfony\Component\Mime\Exception\RfcComplianceException;

/**
 * @author Chris Corbyn
 */
final class ParameterizedHeader extends UnstructuredHeader
{
    /**
     * RFC 2231's definition of a token.
     *
     * @var string
     */
    public const TOKEN_REGEX = '(?:[\x21\x23-\x27\x2A\x2B\x2D\x2E\x30-\x39\x41-\x5A\x5E-\x7E]+)';

    private ?Rfc2231Encoder $encoder = null;
    private array $parameters = [];

    public function __construct(string $name, string $value, array $parameters = [])
    {}

    public function setParameter(string $parameter, ?string $value): void
    {}

    public function getParameter(string $parameter): string
    {}

    /**
     * @param string[] $parameters
     */
    public function setParameters(array $parameters): void
    {}

    /**
     * @return string[]
     */
    public function getParameters(): array
    {}

    public function getBodyAsString(): string
    {}

    /**
     * Generate a list of all tokens in the final header.
     *
     * This doesn't need to be overridden in theory, but it is for implementation
     * reasons to prevent potential breakage of attributes.
     */
    protected function toTokens(?string $string = null): array
    {}

    /**
     * Render an RFC 2047 compliant header parameter from the $name and $value.
     */
    private function createParameter(string $name, string $value): string
    {}

    /**
     * Returns the parameter value from the "=" and beyond.
     *
     * @param string $value to append
     */
    private function getEndOfParameterValue(string $value, bool $encoded = false, bool $firstLine = false): string
    {}
}
