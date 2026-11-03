<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Yaml\Exception;

/**
 * Exception class thrown when an error occurs during parsing.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class ParseException extends RuntimeException
{
    /**
     * @param string      $rawMessage The error message
     * @param int         $parsedLine The line where the error occurred
     * @param string|null $snippet    The snippet of code near the problem
     * @param string|null $parsedFile The file name where the error occurred
     */
    public function __construct(
        private string $rawMessage,
        private int $parsedLine = -1,
        private ?string $snippet = null,
        private ?string $parsedFile = null,
        ?\Throwable $previous = null,
    ) {}

    /**
     * Gets the snippet of code near the error.
     */
    public function getSnippet(): ?string
    {}

    /**
     * Sets the snippet of code near the error.
     */
    public function setSnippet(string $snippet): void
    {}

    /**
     * Gets the filename where the error occurred.
     *
     * This method returns null if a string is parsed.
     */
    public function getParsedFile(): ?string
    {}

    /**
     * Sets the filename where the error occurred.
     */
    public function setParsedFile(string $parsedFile): void
    {}

    /**
     * Gets the line where the error occurred.
     */
    public function getParsedLine(): int
    {}

    /**
     * Sets the line where the error occurred.
     */
    public function setParsedLine(int $parsedLine): void
    {}

    private function updateRepr(): void
    {}
}
