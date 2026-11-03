<?php

namespace Illuminate\Filesystem;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Storage;
use League\Flysystem\PathTraversalDetected;

class ServeFile
{
    /**
     * Create a new invokable controller to serve files.
     */
    public function __construct(
        protected string $disk,
        protected array $config,
        protected bool $isProduction,
    ) {}

    /**
     * Handle the incoming request.
     */
    public function __invoke(Request $request, string $path)
    {}

    /**
     * Determine if the request has a valid signature if applicable.
     */
    protected function hasValidSignature(Request $request): bool
    {}
}
