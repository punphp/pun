<?php

declare(strict_types=1);

namespace GuzzleHttp\Promise;

/**
 * A special exception that is thrown when waiting on a rejected promise.
 *
 * The reason value is available via the getReason() method.
 */
class RejectionException extends \RuntimeException
{
    /** @var mixed Rejection reason. */
    private $reason;

    /**
     * @param mixed       $reason      Rejection reason.
     * @param string|null $description Optional description.
     */
    public function __construct($reason, ?string $description = null)
    {}

    /**
     * Returns the rejection reason.
     *
     * @return mixed
     */
    public function getReason()
    {}
}
