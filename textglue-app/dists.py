from glob import glob
import os.path
import shutil
import re

def mkdirs(path):
    try:
        print(f"MKDIR {path}")
        os.makedirs(path)
    except:
#        import traceback
#        traceback.print_exc()
        print(f"MKDIR {path} - skipped")
        pass

def mkdirs_up(filepath):
    dirpath,_ = os.path.split(filepath)
    print(f"MKDIRS UP {filepath} -> {dirpath}")
    mkdirs(dirpath)

def clean_path(path):
    if path.startswith("dist/"):
        path=path[len("dist/"):]
    return path

def copy(path, target):
    source = path
    destination = change_name(os.path.join(target,clean_path(path)))
    mkdirs_up(destination)
    print(f"COPY {source} -> {destination}")
    shutil.copy(source,destination)
    url=change_name(clean_path(path))
    if url[0]!="/":
        url="/"+url
    url="/textglue"+url #TODO hack...
    return rust_resource(url,destination)

def rust_resource(url,path):
    ext = path.split(".")[-1]
    abspath = os.path.abspath(path)
    mime = dict(
        js   = "application/javascript",
        txt  = "text/plain",
        html = "text/html",
        csv  = "text/csv",
        png  = 'image/png',
        svg  = 'image/svg+xml',
        jpg  = 'image/jpeg',
        jpeg = 'image/jpeg',
        wasm = "application/wasm",
        json = "application/json",
        ico  = "image/x-icon"
        ).get(ext,"text/plain")
    return f"""
            .resource("{url}", |r| r.f(|_r| {{
                const CONTENT: &'static [u8] = include_bytes!("{abspath}");
                HttpResponse::Ok()
                .content_type("{mime}")
                .body(CONTENT)
            }}))
    """

#print (files)

def change_name(name):
    return re.sub(r"\.[a-zA-Z0-9]+\.",".",name)

def process(source, destination, prefix=""):
    rust = "x"
    files = (
        list(glob(f"{source}/*")) +
        list(glob(f"{source}/js/*")) +
        list(glob(f"{source}/css/*")) +
        list(glob(f"{source}/img/*")) +
        list(glob(f"{source}/img/icons/*"))
        )
    for f1 in files:
        print(f"PROCESS {f1}")
        if f1.endswith(".png"):
            rust += copy(f1,destination)
        elif f1.endswith(".ico"):
            rust += copy(f1,destination)
        elif f1.endswith(".svg"):
            rust += copy(f1,destination)
        elif f1.endswith(".wasm"):
            rust += copy(f1,destination)
        elif f1.endswith(".map"):
            print(f"IGNORE {f1}")
        elif f1.endswith(".js") or f1.endswith(".html") or f1.endswith(".css"):
            source = f1
            dst = change_name(os.path.join(destination,clean_path(source)))
            print(f"EDIT {source} -> {dst}")
            mkdirs_up(dst)
            txt = open(source).read()
            for f2 in files:
                if "." not in f2:
                    continue
                if f2.startswith("dist/"):
                    f2c=f2[len("dist/"):]
                original=f2c
                replacement = prefix+change_name(original)
                if original == replacement:
                    continue
                print(f"  REPLACE {original} -> {replacement} in {source}")
                txt = txt.replace(original,replacement)
            print(f"  WRITE {dst}")
            with open(dst,"w") as f:
                f.write(txt)
            url=change_name(clean_path(source))
            if url[0]!="/":
                url="/"+url
            url="/textglue"+url #TODO hack...
            rust += rust_resource(url,dst)
        else:
            print(f"UNKNOWN {f1}")
    return rust

#process("dist","dist1")
rust = process("dist","dist2/textglue","textglue/")
with open("resources.rs","w") as f:
    f.write(rust)