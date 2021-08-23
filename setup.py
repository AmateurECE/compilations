import setuptools

with open('README.md', 'r') as fh:
    long_description = fh.read()

setuptools.setup(
    name='django-compilations',
    version='1.0',
    author='Ethan D. Twardy',
    author_email='ethan.twardy@gmail.com',
    description='Django application for generating video compilations',
    url="https://github.com/AmateurECE/edtwardy-webservices",
    long_description=long_description,
    long_description_content_type='text/markdown',
    packages=['compilations'],
    python_requires='>=3.5',
    install_requires=[
        'django',
        'djangorestframework',
        'requests_oauthlib',
        'bs4',
    ],
    include_package_data=True,
)
