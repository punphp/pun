<?php

declare(strict_types=1);

namespace GuzzleHttp\Exception;

use GuzzleHttp\BodySummarizer;
use GuzzleHttp\BodySummarizerInterface;
use GuzzleHttp\Psr7\DiagnosticValue;
use Psr\Http\Client\RequestExceptionInterface;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;

/**
 * Base exception for request failures associated with a request.
 */
class RequestException extends TransferException implements RequestExceptionInterface
{
    public function __construct(
        string $message,
        RequestInterface $request,
        int $code = 0,
        ?\Throwable $previous = null
    ) {}

    /**
     * Factory method to create a new exception with a normalized error message
     *
     * @param RequestInterface             $request        Request sent
     * @param ResponseInterface|null       $response       Response received, if any
     * @param \Throwable|null              $previous       Previous exception
     * @param BodySummarizerInterface|null $bodySummarizer Optional body summarizer
     */
    public static function create(
        #[\SensitiveParameter]
        RequestInterface $request,
        #[\SensitiveParameter]
        ?ResponseInterface $response = null,
        #[\SensitiveParameter]
        ?\Throwable $previous = null,
        ?BodySummarizerInterface $bodySummarizer = null
    ): self {}
}
