initSidebarItems({"fn":[["download_bytes","Downloads bytes from a URL. This must be called with a tokio runtime somewhere. The caller creates an mpsc channel pair and provides the sender. Progress will be described through it."],["download_to_file","Download a file from a URL. This must be called with a tokio runtime somewhere. Progress will be printed to STDOUT."],["print_download_progress","Print download progress to STDOUT. Pass this the receiver, then call download_to_file or download_bytes with the sender."]]});