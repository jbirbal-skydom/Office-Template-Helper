# rm -r office/
mkdir office/

cp -r /mnt/d/Coding/git/Office-Template-Helper/built/linux/office-template-helper-1.0.0/ ~/office/


cd office/
chmod 644 ./office-template-helper-1.0.0/debian/changelog
chmod 644 ./office-template-helper-1.0.0/debian/control
chmod 644 ./office-template-helper-1.0.0/debian/copyright
chmod 755 ./office-template-helper-1.0.0/debian/postinst
chmod 755 ./office-template-helper-1.0.0/debian/prerm
chmod 755 ./office-template-helper-1.0.0/debian/rules
chmod 755 ./office-template-helper-1.0.0/debian/compat
chmod 644 ./office-template-helper-1.0.0/debian/source/format

tar czvf office-template-helper_1.0.0.orig.tar.gz office-template-helper-1.0.0/

cd office-template-helper-1.0.0/

# debuild -us -uc

#debuild -S -sa