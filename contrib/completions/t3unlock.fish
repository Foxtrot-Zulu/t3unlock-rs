# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_t3unlock_global_optspecs
	string join \n h/help V/version
end

function __fish_t3unlock_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_t3unlock_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_t3unlock_using_subcommand
	set -l cmd (__fish_t3unlock_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c t3unlock -n "__fish_t3unlock_needs_command" -s h -l help -d 'Print help'
complete -c t3unlock -n "__fish_t3unlock_needs_command" -s V -l version -d 'Print version'
complete -c t3unlock -n "__fish_t3unlock_needs_command" -f -a "status" -d 'Show device presence and lock status'
complete -c t3unlock -n "__fish_t3unlock_needs_command" -f -a "unlock" -d 'Unlock the drive (prompts for password if not provided)'
complete -c t3unlock -n "__fish_t3unlock_needs_command" -f -a "doctor" -d 'Diagnose common Linux permission/udev issues'
complete -c t3unlock -n "__fish_t3unlock_needs_command" -f -a "gen-completions" -d 'Generate shell completions'
complete -c t3unlock -n "__fish_t3unlock_needs_command" -f -a "gen-man" -d 'Generate a man page to the given directory'
complete -c t3unlock -n "__fish_t3unlock_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand status" -l vid -d 'USB Vendor ID (hex, e.g. 0x04e8 for Samsung). If omitted, defaults are used' -r
complete -c t3unlock -n "__fish_t3unlock_using_subcommand status" -l pid -d 'USB Product ID (hex). If omitted, defaults are used' -r
complete -c t3unlock -n "__fish_t3unlock_using_subcommand status" -l json -d 'Output JSON'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand status" -s h -l help -d 'Print help'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand unlock" -l vid -d 'USB Vendor ID (hex)' -r
complete -c t3unlock -n "__fish_t3unlock_using_subcommand unlock" -l pid -d 'USB Product ID (hex)' -r
complete -c t3unlock -n "__fish_t3unlock_using_subcommand unlock" -l password -d 'Password (unsafe on shared shells; prefer interactive prompt)' -r
complete -c t3unlock -n "__fish_t3unlock_using_subcommand unlock" -l timeout-ms -d 'USB transfer timeout in milliseconds (default 5000)' -r
complete -c t3unlock -n "__fish_t3unlock_using_subcommand unlock" -l dry-run -d 'Simulate the unlock sequence without touching USB'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand unlock" -s h -l help -d 'Print help'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand doctor" -s h -l help -d 'Print help'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand gen-completions" -s h -l help -d 'Print help'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand gen-man" -s h -l help -d 'Print help'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand help; and not __fish_seen_subcommand_from status unlock doctor gen-completions gen-man help" -f -a "status" -d 'Show device presence and lock status'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand help; and not __fish_seen_subcommand_from status unlock doctor gen-completions gen-man help" -f -a "unlock" -d 'Unlock the drive (prompts for password if not provided)'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand help; and not __fish_seen_subcommand_from status unlock doctor gen-completions gen-man help" -f -a "doctor" -d 'Diagnose common Linux permission/udev issues'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand help; and not __fish_seen_subcommand_from status unlock doctor gen-completions gen-man help" -f -a "gen-completions" -d 'Generate shell completions'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand help; and not __fish_seen_subcommand_from status unlock doctor gen-completions gen-man help" -f -a "gen-man" -d 'Generate a man page to the given directory'
complete -c t3unlock -n "__fish_t3unlock_using_subcommand help; and not __fish_seen_subcommand_from status unlock doctor gen-completions gen-man help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
