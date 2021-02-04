# NOTES

- Workflows
    - Upload MP4 with configuration (sidx, tfdt box, captions? .. think of other sutff later)
    - Stream VOD
    - Live stream RTMP
    - Extract captions from MP4s into WebVTT or some other file type
    - Ad insertion
    - Eventually...
        - Encoding
        - Transcoding

- Tasks
    - Write to local or remote server
    - Read from local or remote server
    - Parse MP4 box
    - Generaete HLS manfiest/s
    - Genearte DASH manifest
    - transmux TS -> MP4 and vice versa
    - Dump info
        - multiple levels of information

* Modules that are responsible for task implementations
* Don't want config files with a shit load of env variables.
* Don't want modules, hooks, tasks to be overwritable. Should have the configurable layer to be at most the workflow layer. Decide what is going to be used and what order, parallel work should be done. Those can have configurtions
* Need a context for any task to work with
* Need a common interface and context type to chain
    * input\<T\> -> MOD_A -> output\<U\> -> input\<U\> -> MOD_B -> output\<W\> 
* Tasks that can be parallelized must fit aboce model

## Macros
- singleton
- injectable
- inject
