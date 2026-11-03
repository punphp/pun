<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler\Exception;

use Symfony\Component\HttpFoundation\Exception\RequestExceptionInterface;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\HttpKernel\Exception\HttpExceptionInterface;
use Symfony\Component\VarDumper\Caster\Caster;
use Symfony\Component\VarDumper\Cloner\Data;
use Symfony\Component\VarDumper\Cloner\Stub;
use Symfony\Component\VarDumper\Cloner\VarCloner;

/**
 * FlattenException wraps a PHP Error or Exception to be able to serialize it.
 *
 * Basically, this class removes all objects from the trace.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class FlattenException
{
    private string $message;
    private string|int $code;
    private ?self $previous = null;
    private array $trace;
    private string $traceAsString;
    private string $class;
    private int $statusCode;
    private string $statusText;
    private array $headers;
    private string $file;
    private int $line;
    private ?string $asString = null;
    private Data $dataRepresentation;

    public static function create(\Exception $exception, ?int $statusCode = null, array $headers = []): static
    {}

    public static function createFromThrowable(\Throwable $exception, ?int $statusCode = null, array $headers = []): static
    {}

    public static function createWithDataRepresentation(\Throwable $throwable, ?int $statusCode = null, array $headers = [], ?VarCloner $cloner = null): static
    {}

    public function toArray(): array
    {}

    public function getStatusCode(): int
    {}

    /**
     * @return $this
     */
    public function setStatusCode(int $code): static
    {}

    public function getHeaders(): array
    {}

    /**
     * @return $this
     */
    public function setHeaders(array $headers): static
    {}

    public function getClass(): string
    {}

    /**
     * @return $this
     */
    public function setClass(string $class): static
    {}

    public function getFile(): string
    {}

    /**
     * @return $this
     */
    public function setFile(string $file): static
    {}

    public function getLine(): int
    {}

    /**
     * @return $this
     */
    public function setLine(int $line): static
    {}

    public function getStatusText(): string
    {}

    /**
     * @return $this
     */
    public function setStatusText(string $statusText): static
    {}

    public function getMessage(): string
    {}

    /**
     * @return $this
     */
    public function setMessage(string $message): static
    {}

    /**
     * @return int|string int most of the time (might be a string with PDOException)
     */
    public function getCode(): int|string
    {}

    /**
     * @return $this
     */
    public function setCode(int|string $code): static
    {}

    public function getPrevious(): ?self
    {}

    /**
     * @return $this
     */
    public function setPrevious(?self $previous): static
    {}

    /**
     * @return self[]
     */
    public function getAllPrevious(): array
    {}

    public function getTrace(): array
    {}

    /**
     * @return $this
     */
    public function setTraceFromThrowable(\Throwable $throwable): static
    {}

    /**
     * @return $this
     */
    public function setTrace(array $trace, ?string $file, ?int $line): static
    {}

    public function getDataRepresentation(): ?Data
    {}

    /**
     * @return $this
     */
    public function setDataRepresentation(Data $data): static
    {}

    private function flattenArgs(array $args, int $level = 0, int &$count = 0): array
    {}

    private function getClassNameFromIncomplete(\__PHP_Incomplete_Class $value): string
    {}

    public function getTraceAsString(): string
    {}

    /**
     * @return $this
     */
    public function setAsString(?string $asString): static
    {}

    public function getAsString(): string
    {}
}
