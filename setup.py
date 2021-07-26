import setuptools

with open('README.md', 'r') as fh:
    long_description = fh.read()

setuptools.setup(
    name='django-compilations',
    version='1.0',
    author='Ethan D. Twardy',
    author_email='ethan.twardy@gmail.com',
    description='Django application for generating video compilations',
    long_description=long_description,
    long_description_content_type='text/markdown',
    packages=setuptools.find_packages(),
    python_requires='>=3.5',
    install_requires=[
        'django',
        'djangorestframework',
    ],
    include_package_data=True,
)
