<?php

namespace Illuminate\Http\Resources;

use Illuminate\Support\Collection;
use JsonSerializable;

class MergeValue
{
    /**
     * The data to be merged.
     *
     * @var array
     */
    public $data;

    /**
     * Create a new merge value instance.
     *
     * @param  \Illuminate\Support\Collection|\JsonSerializable|array  $data
     */
    public function __construct($data)
    {}
}
