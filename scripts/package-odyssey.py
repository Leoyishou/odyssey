"""Package the locally built Odyssey without touching upstream installations."""
from pathlib import Path
import plistlib, shutil, subprocess, hashlib
root=Path(__file__).resolve().parent.parent
app=root/'dist/Odyssey.app'
mac=app/'Contents/MacOS'; res=app/'Contents/Resources'
mac.mkdir(parents=True,exist_ok=True);res.mkdir(parents=True,exist_ok=True)
shutil.copy2(root/'target/release/quiet_md',mac/'odyssey')
icons=root/'dist/Odyssey.iconset';icons.mkdir(exist_ok=True)
for size in (16,32,128,256,512):
 for scale in (1,2):
  dest=icons/f'icon_{size}x{size}{"@2x" if scale==2 else ""}.png'
  subprocess.run(['sips','-z',str(size*scale),str(size*scale),str(root/'assets/branding/app-icon.png'),'--out',str(dest)],check=True,stdout=subprocess.DEVNULL)
subprocess.run(['iconutil','-c','icns',str(icons),'-o',str(res/'Odyssey.icns')],check=True)
info={'CFBundleName':'Odyssey','CFBundleDisplayName':'Odyssey','CFBundleIdentifier':'app.odyssey.local','CFBundleExecutable':'odyssey','CFBundleIconFile':'Odyssey.icns','CFBundlePackageType':'APPL','CFBundleShortVersionString':'0.1.5','CFBundleVersion':'6','NSHighResolutionCapable':True,'LSMinimumSystemVersion':'13.0','NSDocumentsFolderUsageDescription':'Odyssey opens only the local note folders you select.'}
(app/'Contents/Info.plist').write_bytes(plistlib.dumps(info))
for f in ('LICENSE','THIRD_PARTY.md'):
 shutil.copy2(root/f,res/f)
subprocess.run(['codesign','--force','--sign','-',str(app)],check=True)
subprocess.run(['codesign','--verify','--deep','--strict',str(app)],check=True)
(root/'dist/odyssey-binary.sha256').write_text(hashlib.sha256((mac/'odyssey').read_bytes()).hexdigest()+'  Odyssey.app/Contents/MacOS/odyssey\n')
print(app)
