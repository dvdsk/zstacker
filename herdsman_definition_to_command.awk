# typescript fragment to translate needs to start with empty line

BEGIN { RS="[[:space:]]+},"; FS="\n" }
BEGIN { print "#![allow(dead_code)]" }
BEGIN { print "use serde::{Serialize, Deserialize};\n" }
BEGIN { print "use super::{Command, CommandType, Status, Subsystem, IeeeAddr};\n" }
{ 	
	name = $3; 
	gsub(/ +name: '/, "", name); 
	gsub(/',/, "", name);  
	name = toupper(substr(name, 1, 1)) substr(name, 2) 
}
{ id = $4; gsub(/ +ID: /, "", id); gsub(/,/, "", id) }
{ type = $5; gsub(/ +type: /, "", type); gsub(/,/, "", type); gsub(/\./, "::", type) }

# request fields
{ 
	started = 0
	delete req_field_names; delete req_field_types; 
	delete req_commented; delete req_comments;
	for(i=2; i<99; i++) {
		if (length($i) == 0) { break }
		if ($i ~ /request: \[$/) { started = 1; continue }
		if ($i ~ /request: \[{/) { started = 1 }

		if (started) {
			if ($i ~ / ],$/) { break }
			if ($i ~ /response: \[/) { break }
			if ($i ~ /\/\//) { req_commented[i] = 1 }
			if ($i ~ /}, \/\//) { req_comments[i] = gensub(/.+}, /, "//", "g", $i) }
			if ($i !~ /{name/) { req_comments[i] = $i; continue }
			req_field_names[i] = gensub(/.+{name: '(.+)', .+/, "\\1", "g", $i);
			req_field_types[i] = gensub(/.+ParameterType.(.+)}.+/, "\\1", "g", $i);
		}
	}
	# for (i in req_field_names) print req_field_types[i]
}

# response fields
{
	started = 0
	rsp_fields = 0
	delete rsp_field_names; delete rsp_field_types; 
	delete rsp_commented; delete rsp_comments;
	for(i=2 ; i<99; i++) {
		if (length($i) == 0) { break }
		if ($i ~ /response: \[$/) { started = 1; continue }
		if ($i ~ /response: \[/) { started = 1 }

		if (started) {
			if ($i ~ / ],$/) { break }
			if ($i ~ / },$/) { break }
			if ($i ~ /\/\//) { rsp_commented[i] = 1 }
			if ($i !~ /{name/) { rsp_comments[i] = $i; continue }
			rsp_field_names[rsp_fields] = gensub(/.+{name: '(.+)', .+/, "\\1", "g", $i);
			rsp_field_types[rsp_fields] = gensub(/.+ParameterType.(.+)}]?,/, "\\1", "g", $i);
			rsp_fields += 1
		}
	}
}

function translate_types(list)
{
	for (i in list) {
		switch (list[i]) {
			case /LIST_UINT[[:digit:]]+/:
				list[i] = "Vec<" gensub("LIST_UINT", "u", "g", list[i]) ">"
				break
			case /UINT[[:digit:]]+/:
				list[i] = gensub("UINT", "u", "g", list[i])
				break
			case "INT8":
				list[i] = "i8"
				break
			case "BUFFER":
				list[i] = "Vec<u8>"
				break
			case "BUFFER8":
				list[i] = "[u8; 8]"
				break
			case "BUFFER16":
				list[i] = "[u8; 16]"
				break
			case "BUFFER32":
				list[i] = "[u8; 32]"
				break
			case "IEEEADDR":
				list[i] = "IeeeAddr"
				break
			case "LIST_ASSOC_DEV":
				list[i] = "Vec<u16>"
				break
			case "LIST_NETWORK":
				list[i] = "compile_error!(\"needs custom derive with Network type\")"
				break
			case "LIST_NEIGHBOR_LQI":
				list[i] = "compile_error!(\"needs custom derive with NeighborLqi type\")"
				break
			case "LIST_ROUTING_TABLE":
				list[i] = "RoutingTable"
				break
			case "LIST_BIND_TABLE":
				list[i] = "BindTable"
				break
			default:
				print "error can not translate type: '" list[i] "'"
				print "for field " list[i]
				exit 
		}
	}
}

function translate_names(list)
{
	for (i in list) {
		switch (list[i]) {
			case "type":
				list[i] = "ty"
				break
			default:
		}
	}
}

function remove_list_counts(name_list, type_list)
{
	prev_idx = "None"
	for (i in name_list) {
		if (prev_idx == "None") {
			prev_idx = i
			continue
		}

		if (gensub(/count/, "", "g", name_list[prev_idx]) == gensub(/list/, "", "g", name_list[i])) {
			delete name_list[prev_idx]
			delete type_list[prev_idx]
		} else if (gensub(/s$/, "", "g", gensub(/num/, "", "g", name_list[prev_idx])) \
			== gensub(/list/, "", "g", name_list[i])) {
			delete name_list[prev_idx]
			delete type_list[prev_idx]
		}
		prev_idx = i
	}
}

{
	translate_types(req_field_types)
	translate_types(rsp_field_types)
	translate_names(req_field_names)
	translate_names(rsp_field_names)
	remove_list_counts(req_field_names, req_field_types)
	remove_list_counts(rsp_field_names, rsp_field_types)
}

function first(list_a, list_b) 
{
	min = 9999
	for (i in list_a) {
		if (i+0 < min+0) {
			min = i
		}
	}
	for (i in list_b) {
		if (i+0 < min+0) {
			min = i
		}
	}
	return min
}

# print struct and cmd impl
{ 
	if (length(name) > 0) {
		print "#[derive(Debug, Clone, Serialize)]"
		print "struct " name " {";
		start = first(req_field_names, req_comments)
		end = start + length(req_field_names) + length(req_comments)
		for (i = start + 0; i < end + 1; i++) {
			if (i in req_comments) {
				print req_comments[i]
			}
			if (i in req_field_names) {
				if (i in req_commented) {
					print "\t// " req_field_names[i] ": " req_field_types[i]  ","
				} else {
					print "\t" req_field_names[i] ": " req_field_types[i]  ","
				}
			}
		}
		print "}\n"

        # print reply struct if any
		if (rsp_fields == 0) {
			reply = "()"
		} else if (rsp_fields == 1 \
			   && rsp_field_names[0] == "status" \
			   && rsp_field_types[0] == "u8") {
			reply = "Status"
		} else {
			reply_name = name "Reply"
			print "#[derive(Debug, Clone, Deserialize)]"
			print "struct " reply_name " {"
			start = first(rsp_field_names, rsp_comments)
			end = start + length(rsp_field_names) + length(rsp_comments)
			for (i = start + 0; i < end + 1; i++) {
				if (i in rsp_comments) {
					print rsp_comments[i]
				}
				if (i in rsp_field_names) {
					if (i in rsp_commented) {
						print "\t// " rsp_field_names[i] ": " rsp_field_types[i]  ","
					} else {
						print "\t" rsp_field_names[i] ": " rsp_field_types[i]  ","
					}
				}
			}
			print "}\n"
			reply = reply_name
		}

        # print command impl
		print "impl Command for " name " {"
		print "\tconst ID: u8 = " id ";"
		print "\tconst TYPE: CommandType = " type ";"
		print "\tconst SUBSYSTEM: Subsystem = Subsystem::App" ";"
		print "\ttype Reply = " reply ";"
		print "}\n"
	}
}
