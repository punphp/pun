<?php

declare(strict_types=1);

namespace GuzzleHttp;

use Psr\Http\Message\MessageInterface;

final class BodySummarizer implements BodySummarizerInterface
{
    private ?int $truncateAt;

    public function __construct(?int $truncateAt = null)
    {}

    /**
     * Returns a summarized message body.
     */
    public function summarize(MessageInterface $message): ?string
    {}
}
