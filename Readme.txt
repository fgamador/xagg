xagg - Aggregate expenses

Archive old CSV and config.json files in input subfolders.
Download account CSV files into input subfolders (see Readme.txt files there).
Remove extra CSV header lines as needed. Should be only a single header line.
Update config.json as needed to match CSV format. See https://docs.rs/chrono/latest/chrono/format/strftime/index.html for date format.

% cargo run > categories.json
Notepad++ > Plugins > JSON Viewer > Format JSON
% mv categories.json ..\..\xagg-zoomable-icicle\files
Overwrite file with long hex name with contents of categories.json.
In ..\..\xagg-zoomable-icicle: % http-server
Visit http://localhost:8080/

Update input\rules.json as needed to refine categories, then regenerate categories.json.
