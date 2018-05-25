init = function(args)
    request_uri = args[1]
    depth = tonumber(args[2]) or 1
 
    -- io.write("num".. depth .."\n")
 
    local r = {}
    for i=1,depth do
      r[i] = wrk.format(nil, request_uri)
    end
    req = table.concat(r)
 end
 
 request = function()
    return req
 end