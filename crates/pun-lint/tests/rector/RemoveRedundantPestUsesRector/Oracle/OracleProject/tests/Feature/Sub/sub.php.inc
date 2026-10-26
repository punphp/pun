<?php

use Tests\Fixture\SampleTrait;
use Illuminate\Foundation\Testing\RefreshDatabase;

uses(SampleTrait::class, RefreshDatabase::class, Missing::class);
-----
<?php

use Tests\Fixture\SampleTrait;
use Illuminate\Foundation\Testing\RefreshDatabase;

uses(Missing::class);
