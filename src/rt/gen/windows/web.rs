use crate::prelude::*;
DEFINE_IID!(IID_IUriToStreamResolver, 2964039786, 39659, 19770, 149, 144, 0, 62, 60, 167, 226, 144);
RT_INTERFACE!{interface IUriToStreamResolver(IUriToStreamResolverVtbl): IInspectable(IInspectableVtbl) [IID_IUriToStreamResolver] {
    #[cfg(feature="windows-storage")] fn UriToStreamAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<super::storage::streams::IInputStream>) -> HRESULT
}}
impl ComPtr<IUriToStreamResolver> {
    #[cfg(feature="windows-storage")] #[inline] pub fn uri_to_stream_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperation<super::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).UriToStreamAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class WebError}
impl RtActivatable<IWebErrorStatics> for WebError {}
impl WebError {
    #[inline] pub fn get_status(hresult: i32) -> Result<WebErrorStatus> {
        <Self as RtActivatable<IWebErrorStatics>>::get_activation_factory().get_status(hresult)
    }
}
DEFINE_CLSID!(WebError(&[87,105,110,100,111,119,115,46,87,101,98,46,87,101,98,69,114,114,111,114,0]) [CLSID_WebError]);
DEFINE_IID!(IID_IWebErrorStatics, 4267796326, 48935, 16484, 135, 183, 101, 99, 187, 17, 206, 46);
RT_INTERFACE!{static interface IWebErrorStatics(IWebErrorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWebErrorStatics] {
    fn GetStatus(&self, hresult: i32, out: *mut WebErrorStatus) -> HRESULT
}}
impl ComPtr<IWebErrorStatics> {
    #[inline] pub fn get_status(&self, hresult: i32) -> Result<WebErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetStatus)(self.deref() as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebErrorStatus: i32 {
    Unknown = 0, CertificateCommonNameIsIncorrect = 1, CertificateExpired = 2, CertificateContainsErrors = 3, CertificateRevoked = 4, CertificateIsInvalid = 5, ServerUnreachable = 6, Timeout = 7, ErrorHttpInvalidServerResponse = 8, ConnectionAborted = 9, ConnectionReset = 10, Disconnected = 11, HttpToHttpsOnRedirection = 12, HttpsToHttpOnRedirection = 13, CannotConnect = 14, HostNameNotResolved = 15, OperationCanceled = 16, RedirectFailed = 17, UnexpectedStatusCode = 18, UnexpectedRedirection = 19, UnexpectedClientError = 20, UnexpectedServerError = 21, InsufficientRangeSupport = 22, MissingContentLengthSupport = 23, MultipleChoices = 300, MovedPermanently = 301, Found = 302, SeeOther = 303, NotModified = 304, UseProxy = 305, TemporaryRedirect = 307, BadRequest = 400, Unauthorized = 401, PaymentRequired = 402, Forbidden = 403, NotFound = 404, MethodNotAllowed = 405, NotAcceptable = 406, ProxyAuthenticationRequired = 407, RequestTimeout = 408, Conflict = 409, Gone = 410, LengthRequired = 411, PreconditionFailed = 412, RequestEntityTooLarge = 413, RequestUriTooLong = 414, UnsupportedMediaType = 415, RequestedRangeNotSatisfiable = 416, ExpectationFailed = 417, InternalServerError = 500, NotImplemented = 501, BadGateway = 502, ServiceUnavailable = 503, GatewayTimeout = 504, HttpVersionNotSupported = 505,
}}
pub mod atompub { // Windows.Web.AtomPub
use crate::prelude::*;
DEFINE_IID!(IID_IAtomPubClient, 892939320, 52717, 19788, 150, 55, 5, 241, 92, 28, 148, 6);
RT_INTERFACE!{interface IAtomPubClient(IAtomPubClientVtbl): IInspectable(IInspectableVtbl) [IID_IAtomPubClient] {
    fn RetrieveServiceDocumentAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<ServiceDocument, super::syndication::RetrievalProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RetrieveMediaResourceAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, super::syndication::RetrievalProgress>) -> HRESULT,
    fn RetrieveResourceAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::RetrievalProgress>) -> HRESULT,
    fn CreateResourceAsync(&self, uri: *mut foundation::Uri, description: HSTRING, item: *mut super::syndication::SyndicationItem, out: *mut *mut foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::TransferProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateMediaResourceAsync(&self, uri: *mut foundation::Uri, mediaType: HSTRING, description: HSTRING, mediaStream: *mut super::super::storage::streams::IInputStream, out: *mut *mut foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::TransferProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn UpdateMediaResourceAsync(&self, uri: *mut foundation::Uri, mediaType: HSTRING, mediaStream: *mut super::super::storage::streams::IInputStream, out: *mut *mut foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn UpdateResourceAsync(&self, uri: *mut foundation::Uri, item: *mut super::syndication::SyndicationItem, out: *mut *mut foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn UpdateResourceItemAsync(&self, item: *mut super::syndication::SyndicationItem, out: *mut *mut foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn DeleteResourceAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn DeleteResourceItemAsync(&self, item: *mut super::syndication::SyndicationItem, out: *mut *mut foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>) -> HRESULT,
    fn CancelAsyncOperations(&self) -> HRESULT
}}
impl ComPtr<IAtomPubClient> {
    #[inline] pub fn retrieve_service_document_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<ServiceDocument, super::syndication::RetrievalProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).RetrieveServiceDocumentAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn retrieve_media_resource_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, super::syndication::RetrievalProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).RetrieveMediaResourceAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn retrieve_resource_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::RetrievalProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).RetrieveResourceAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_resource_async(&self, uri: &ComPtr<foundation::Uri>, description: &HStringArg, item: &ComPtr<super::syndication::SyndicationItem>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::TransferProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateResourceAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, description.get(), item.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_media_resource_async(&self, uri: &ComPtr<foundation::Uri>, mediaType: &HStringArg, description: &HStringArg, mediaStream: &ComPtr<super::super::storage::streams::IInputStream>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::syndication::SyndicationItem, super::syndication::TransferProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateMediaResourceAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, mediaType.get(), description.get(), mediaStream.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn update_media_resource_async(&self, uri: &ComPtr<foundation::Uri>, mediaType: &HStringArg, mediaStream: &ComPtr<super::super::storage::streams::IInputStream>) -> Result<ComPtr<foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).UpdateMediaResourceAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, mediaType.get(), mediaStream.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_resource_async(&self, uri: &ComPtr<foundation::Uri>, item: &ComPtr<super::syndication::SyndicationItem>) -> Result<ComPtr<foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).UpdateResourceAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, item.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_resource_item_async(&self, item: &ComPtr<super::syndication::SyndicationItem>) -> Result<ComPtr<foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).UpdateResourceItemAsync)(self.deref() as *const _ as *mut _, item.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_resource_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).DeleteResourceAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_resource_item_async(&self, item: &ComPtr<super::syndication::SyndicationItem>) -> Result<ComPtr<foundation::IAsyncActionWithProgress<super::syndication::TransferProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).DeleteResourceItemAsync)(self.deref() as *const _ as *mut _, item.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn cancel_async_operations(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).CancelAsyncOperations)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class AtomPubClient: IAtomPubClient}
impl RtActivatable<IAtomPubClientFactory> for AtomPubClient {}
impl RtActivatable<IActivationFactory> for AtomPubClient {}
impl AtomPubClient {
    #[cfg(feature="windows-security")] #[inline] pub fn create_atom_pub_client_with_credentials(serverCredential: &ComPtr<super::super::security::credentials::PasswordCredential>) -> Result<ComPtr<AtomPubClient>> {
        <Self as RtActivatable<IAtomPubClientFactory>>::get_activation_factory().create_atom_pub_client_with_credentials(serverCredential)
    }
}
DEFINE_CLSID!(AtomPubClient(&[87,105,110,100,111,119,115,46,87,101,98,46,65,116,111,109,80,117,98,46,65,116,111,109,80,117,98,67,108,105,101,110,116,0]) [CLSID_AtomPubClient]);
DEFINE_IID!(IID_IAtomPubClientFactory, 1238716434, 22475, 19422, 171, 159, 38, 16, 177, 114, 119, 123);
RT_INTERFACE!{static interface IAtomPubClientFactory(IAtomPubClientFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IAtomPubClientFactory] {
    #[cfg(feature="windows-security")] fn CreateAtomPubClientWithCredentials(&self, serverCredential: *mut super::super::security::credentials::PasswordCredential, out: *mut *mut AtomPubClient) -> HRESULT
}}
impl ComPtr<IAtomPubClientFactory> {
    #[cfg(feature="windows-security")] #[inline] pub fn create_atom_pub_client_with_credentials(&self, serverCredential: &ComPtr<super::super::security::credentials::PasswordCredential>) -> Result<ComPtr<AtomPubClient>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateAtomPubClientWithCredentials)(self.deref() as *const _ as *mut _, serverCredential.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IResourceCollection, 2136987145, 48264, 16852, 136, 250, 61, 230, 112, 77, 66, 142);
RT_INTERFACE!{interface IResourceCollection(IResourceCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IResourceCollection] {
    fn get_Title(&self, out: *mut *mut super::syndication::ISyndicationText) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Categories(&self, out: *mut *mut foundation::collections::IVectorView<super::syndication::SyndicationCategory>) -> HRESULT,
    fn get_Accepts(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl ComPtr<IResourceCollection> {
    #[inline] pub fn get_title(&self) -> Result<Option<ComPtr<super::syndication::ISyndicationText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Title)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_categories(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::syndication::SyndicationCategory>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Categories)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_accepts(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Accepts)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ResourceCollection: IResourceCollection}
DEFINE_IID!(IID_IServiceDocument, 2340341617, 10931, 19902, 139, 204, 119, 143, 146, 183, 94, 81);
RT_INTERFACE!{interface IServiceDocument(IServiceDocumentVtbl): IInspectable(IInspectableVtbl) [IID_IServiceDocument] {
    fn get_Workspaces(&self, out: *mut *mut foundation::collections::IVectorView<Workspace>) -> HRESULT
}}
impl ComPtr<IServiceDocument> {
    #[inline] pub fn get_workspaces(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<Workspace>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Workspaces)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ServiceDocument: IServiceDocument}
DEFINE_IID!(IID_IWorkspace, 3021841979, 42168, 16438, 137, 197, 131, 195, 18, 102, 186, 73);
RT_INTERFACE!{interface IWorkspace(IWorkspaceVtbl): IInspectable(IInspectableVtbl) [IID_IWorkspace] {
    fn get_Title(&self, out: *mut *mut super::syndication::ISyndicationText) -> HRESULT,
    fn get_Collections(&self, out: *mut *mut foundation::collections::IVectorView<ResourceCollection>) -> HRESULT
}}
impl ComPtr<IWorkspace> {
    #[inline] pub fn get_title(&self) -> Result<Option<ComPtr<super::syndication::ISyndicationText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Title)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_collections(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<ResourceCollection>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Collections)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Workspace: IWorkspace}
} // Windows.Web.AtomPub
pub mod http { // Windows.Web.Http
use crate::prelude::*;
RT_CLASS!{class HttpBufferContent: IHttpContent}
impl RtActivatable<IHttpBufferContentFactory> for HttpBufferContent {}
impl HttpBufferContent {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_buffer(content: &ComPtr<super::super::storage::streams::IBuffer>) -> Result<ComPtr<HttpBufferContent>> {
        <Self as RtActivatable<IHttpBufferContentFactory>>::get_activation_factory().create_from_buffer(content)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_buffer_with_offset(content: &ComPtr<super::super::storage::streams::IBuffer>, offset: u32, count: u32) -> Result<ComPtr<HttpBufferContent>> {
        <Self as RtActivatable<IHttpBufferContentFactory>>::get_activation_factory().create_from_buffer_with_offset(content, offset, count)
    }
}
DEFINE_CLSID!(HttpBufferContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,66,117,102,102,101,114,67,111,110,116,101,110,116,0]) [CLSID_HttpBufferContent]);
DEFINE_IID!(IID_IHttpBufferContentFactory, 3156263315, 50207, 20471, 145, 35, 100, 53, 115, 110, 173, 194);
RT_INTERFACE!{static interface IHttpBufferContentFactory(IHttpBufferContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpBufferContentFactory] {
    #[cfg(feature="windows-storage")] fn CreateFromBuffer(&self, content: *mut super::super::storage::streams::IBuffer, out: *mut *mut HttpBufferContent) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateFromBufferWithOffset(&self, content: *mut super::super::storage::streams::IBuffer, offset: u32, count: u32, out: *mut *mut HttpBufferContent) -> HRESULT
}}
impl ComPtr<IHttpBufferContentFactory> {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_buffer(&self, content: &ComPtr<super::super::storage::streams::IBuffer>) -> Result<ComPtr<HttpBufferContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromBuffer)(self.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_buffer_with_offset(&self, content: &ComPtr<super::super::storage::streams::IBuffer>, offset: u32, count: u32) -> Result<ComPtr<HttpBufferContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromBufferWithOffset)(self.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _, offset, count, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpClient, 2144997713, 13684, 18560, 168, 186, 230, 177, 224, 6, 31, 61);
RT_INTERFACE!{interface IHttpClient(IHttpClientVtbl): IInspectable(IInspectableVtbl) [IID_IHttpClient] {
    fn DeleteAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn GetAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn GetWithOptionAsync(&self, uri: *mut foundation::Uri, completionOption: HttpCompletionOption, out: *mut *mut foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetBufferAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<super::super::storage::streams::IBuffer, HttpProgress>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetInputStreamAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, HttpProgress>) -> HRESULT,
    fn GetStringAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<HString, HttpProgress>) -> HRESULT,
    fn PostAsync(&self, uri: *mut foundation::Uri, content: *mut IHttpContent, out: *mut *mut foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn PutAsync(&self, uri: *mut foundation::Uri, content: *mut IHttpContent, out: *mut *mut foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn SendRequestAsync(&self, request: *mut HttpRequestMessage, out: *mut *mut foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn SendRequestWithOptionAsync(&self, request: *mut HttpRequestMessage, completionOption: HttpCompletionOption, out: *mut *mut foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>) -> HRESULT,
    fn get_DefaultRequestHeaders(&self, out: *mut *mut headers::HttpRequestHeaderCollection) -> HRESULT
}}
impl ComPtr<IHttpClient> {
    #[inline] pub fn delete_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).DeleteAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_with_option_async(&self, uri: &ComPtr<foundation::Uri>, completionOption: HttpCompletionOption) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetWithOptionAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, completionOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_buffer_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::super::storage::streams::IBuffer, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetBufferAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_input_stream_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetInputStreamAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_string_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HString, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetStringAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn post_async(&self, uri: &ComPtr<foundation::Uri>, content: &ComPtr<IHttpContent>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).PostAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn put_async(&self, uri: &ComPtr<foundation::Uri>, content: &ComPtr<IHttpContent>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).PutAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn send_request_async(&self, request: &ComPtr<HttpRequestMessage>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SendRequestAsync)(self.deref() as *const _ as *mut _, request.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn send_request_with_option_async(&self, request: &ComPtr<HttpRequestMessage>, completionOption: HttpCompletionOption) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HttpResponseMessage, HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SendRequestWithOptionAsync)(self.deref() as *const _ as *mut _, request.deref() as *const _ as *mut _, completionOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_default_request_headers(&self) -> Result<Option<ComPtr<headers::HttpRequestHeaderCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_DefaultRequestHeaders)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpClient: IHttpClient}
impl RtActivatable<IHttpClientFactory> for HttpClient {}
impl RtActivatable<IActivationFactory> for HttpClient {}
impl HttpClient {
    #[inline] pub fn create(filter: &ComPtr<filters::IHttpFilter>) -> Result<ComPtr<HttpClient>> {
        <Self as RtActivatable<IHttpClientFactory>>::get_activation_factory().create(filter)
    }
}
DEFINE_CLSID!(HttpClient(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,67,108,105,101,110,116,0]) [CLSID_HttpClient]);
DEFINE_IID!(IID_IHttpClientFactory, 3272363722, 58362, 20377, 175, 180, 99, 204, 101, 0, 148, 98);
RT_INTERFACE!{static interface IHttpClientFactory(IHttpClientFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpClientFactory] {
    fn Create(&self, filter: *mut filters::IHttpFilter, out: *mut *mut HttpClient) -> HRESULT
}}
impl ComPtr<IHttpClientFactory> {
    #[inline] pub fn create(&self, filter: &ComPtr<filters::IHttpFilter>) -> Result<ComPtr<HttpClient>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, filter.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum HttpCompletionOption: i32 {
    ResponseContentRead = 0, ResponseHeadersRead = 1,
}}
DEFINE_IID!(IID_IHttpContent, 1796514881, 64423, 19410, 175, 10, 131, 157, 231, 194, 149, 218);
RT_INTERFACE!{interface IHttpContent(IHttpContentVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContent] {
    fn get_Headers(&self, out: *mut *mut headers::HttpContentHeaderCollection) -> HRESULT,
    fn BufferAllAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<u64, u64>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ReadAsBufferAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<super::super::storage::streams::IBuffer, u64>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ReadAsInputStreamAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, u64>) -> HRESULT,
    fn ReadAsStringAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<HString, u64>) -> HRESULT,
    fn TryComputeLength(&self, length: *mut u64, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-storage")] fn WriteToStreamAsync(&self, outputStream: *mut super::super::storage::streams::IOutputStream, out: *mut *mut foundation::IAsyncOperationWithProgress<u64, u64>) -> HRESULT
}}
impl ComPtr<IHttpContent> {
    #[inline] pub fn get_headers(&self) -> Result<Option<ComPtr<headers::HttpContentHeaderCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Headers)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn buffer_all_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<u64, u64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).BufferAllAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn read_as_buffer_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::super::storage::streams::IBuffer, u64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).ReadAsBufferAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn read_as_input_stream_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::super::storage::streams::IInputStream, u64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).ReadAsInputStreamAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_as_string_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<HString, u64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).ReadAsStringAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_compute_length(&self) -> Result<(u64, bool)> { unsafe { 
        let mut length = zeroed(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryComputeLength)(self.deref() as *const _ as *mut _, &mut length, &mut out);
        if hr == S_OK { Ok((length, out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn write_to_stream_async(&self, outputStream: &ComPtr<super::super::storage::streams::IOutputStream>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<u64, u64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).WriteToStreamAsync)(self.deref() as *const _ as *mut _, outputStream.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpCookie, 525633762, 52269, 18297, 134, 167, 136, 241, 6, 135, 210, 73);
RT_INTERFACE!{interface IHttpCookie(IHttpCookieVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookie] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Domain(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Expires(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn put_Expires(&self, value: *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_HttpOnly(&self, out: *mut bool) -> HRESULT,
    fn put_HttpOnly(&self, value: bool) -> HRESULT,
    fn get_Secure(&self, out: *mut bool) -> HRESULT,
    fn put_Secure(&self, value: bool) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<IHttpCookie> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Domain)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Path)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_expires(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Expires)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_expires(&self, value: &ComPtr<foundation::IReference<foundation::DateTime>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Expires)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_http_only(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_HttpOnly)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_http_only(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_HttpOnly)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_secure(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Secure)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_secure(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Secure)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Value)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpCookie: IHttpCookie}
impl RtActivatable<IHttpCookieFactory> for HttpCookie {}
impl HttpCookie {
    #[inline] pub fn create(name: &HStringArg, domain: &HStringArg, path: &HStringArg) -> Result<ComPtr<HttpCookie>> {
        <Self as RtActivatable<IHttpCookieFactory>>::get_activation_factory().create(name, domain, path)
    }
}
DEFINE_CLSID!(HttpCookie(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,67,111,111,107,105,101,0]) [CLSID_HttpCookie]);
RT_CLASS!{class HttpCookieCollection: foundation::collections::IVectorView<HttpCookie>}
DEFINE_IID!(IID_IHttpCookieFactory, 1778746793, 37660, 19665, 169, 109, 194, 23, 1, 120, 92, 95);
RT_INTERFACE!{static interface IHttpCookieFactory(IHttpCookieFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookieFactory] {
    fn Create(&self, name: HSTRING, domain: HSTRING, path: HSTRING, out: *mut *mut HttpCookie) -> HRESULT
}}
impl ComPtr<IHttpCookieFactory> {
    #[inline] pub fn create(&self, name: &HStringArg, domain: &HStringArg, path: &HStringArg) -> Result<ComPtr<HttpCookie>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, name.get(), domain.get(), path.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpCookieManager, 2051217280, 52559, 20055, 168, 74, 91, 10, 83, 214, 187, 150);
RT_INTERFACE!{interface IHttpCookieManager(IHttpCookieManagerVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookieManager] {
    fn SetCookie(&self, cookie: *mut HttpCookie, out: *mut bool) -> HRESULT,
    fn SetCookieWithThirdParty(&self, cookie: *mut HttpCookie, thirdParty: bool, out: *mut bool) -> HRESULT,
    fn DeleteCookie(&self, cookie: *mut HttpCookie) -> HRESULT,
    fn GetCookies(&self, uri: *mut foundation::Uri, out: *mut *mut HttpCookieCollection) -> HRESULT
}}
impl ComPtr<IHttpCookieManager> {
    #[inline] pub fn set_cookie(&self, cookie: &ComPtr<HttpCookie>) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).SetCookie)(self.deref() as *const _ as *mut _, cookie.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_cookie_with_third_party(&self, cookie: &ComPtr<HttpCookie>, thirdParty: bool) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).SetCookieWithThirdParty)(self.deref() as *const _ as *mut _, cookie.deref() as *const _ as *mut _, thirdParty, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn delete_cookie(&self, cookie: &ComPtr<HttpCookie>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).DeleteCookie)(self.deref() as *const _ as *mut _, cookie.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cookies(&self, uri: &ComPtr<foundation::Uri>) -> Result<Option<ComPtr<HttpCookieCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetCookies)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpCookieManager: IHttpCookieManager}
RT_CLASS!{class HttpFormUrlEncodedContent: IHttpContent}
impl RtActivatable<IHttpFormUrlEncodedContentFactory> for HttpFormUrlEncodedContent {}
impl HttpFormUrlEncodedContent {
    #[inline] pub fn create(content: &ComPtr<foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, HString>>>) -> Result<ComPtr<HttpFormUrlEncodedContent>> {
        <Self as RtActivatable<IHttpFormUrlEncodedContentFactory>>::get_activation_factory().create(content)
    }
}
DEFINE_CLSID!(HttpFormUrlEncodedContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,70,111,114,109,85,114,108,69,110,99,111,100,101,100,67,111,110,116,101,110,116,0]) [CLSID_HttpFormUrlEncodedContent]);
DEFINE_IID!(IID_IHttpFormUrlEncodedContentFactory, 1139807116, 12147, 17154, 181, 243, 234, 233, 35, 138, 94, 1);
RT_INTERFACE!{static interface IHttpFormUrlEncodedContentFactory(IHttpFormUrlEncodedContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpFormUrlEncodedContentFactory] {
    fn Create(&self, content: *mut foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, HString>>, out: *mut *mut HttpFormUrlEncodedContent) -> HRESULT
}}
impl ComPtr<IHttpFormUrlEncodedContentFactory> {
    #[inline] pub fn create(&self, content: &ComPtr<foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, HString>>>) -> Result<ComPtr<HttpFormUrlEncodedContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMethod, 1921859618, 28685, 20448, 175, 165, 64, 41, 156, 88, 219, 253);
RT_INTERFACE!{interface IHttpMethod(IHttpMethodVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMethod] {
    fn get_Method(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHttpMethod> {
    #[inline] pub fn get_method(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Method)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpMethod: IHttpMethod}
impl RtActivatable<IHttpMethodFactory> for HttpMethod {}
impl RtActivatable<IHttpMethodStatics> for HttpMethod {}
impl HttpMethod {
    #[inline] pub fn create(method: &HStringArg) -> Result<ComPtr<HttpMethod>> {
        <Self as RtActivatable<IHttpMethodFactory>>::get_activation_factory().create(method)
    }
    #[inline] pub fn get_delete() -> Result<Option<ComPtr<HttpMethod>>> {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_delete()
    }
    #[inline] pub fn get_get() -> Result<Option<ComPtr<HttpMethod>>> {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_get()
    }
    #[inline] pub fn get_head() -> Result<Option<ComPtr<HttpMethod>>> {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_head()
    }
    #[inline] pub fn get_options() -> Result<Option<ComPtr<HttpMethod>>> {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_options()
    }
    #[inline] pub fn get_patch() -> Result<Option<ComPtr<HttpMethod>>> {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_patch()
    }
    #[inline] pub fn get_post() -> Result<Option<ComPtr<HttpMethod>>> {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_post()
    }
    #[inline] pub fn get_put() -> Result<Option<ComPtr<HttpMethod>>> {
        <Self as RtActivatable<IHttpMethodStatics>>::get_activation_factory().get_put()
    }
}
DEFINE_CLSID!(HttpMethod(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,77,101,116,104,111,100,0]) [CLSID_HttpMethod]);
DEFINE_IID!(IID_IHttpMethodFactory, 1011994893, 14039, 16632, 168, 109, 231, 89, 202, 242, 248, 63);
RT_INTERFACE!{static interface IHttpMethodFactory(IHttpMethodFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMethodFactory] {
    fn Create(&self, method: HSTRING, out: *mut *mut HttpMethod) -> HRESULT
}}
impl ComPtr<IHttpMethodFactory> {
    #[inline] pub fn create(&self, method: &HStringArg) -> Result<ComPtr<HttpMethod>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, method.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMethodStatics, 1691447792, 55706, 16723, 141, 198, 214, 140, 196, 204, 227, 23);
RT_INTERFACE!{static interface IHttpMethodStatics(IHttpMethodStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMethodStatics] {
    fn get_Delete(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Get(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Head(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Options(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Patch(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Post(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn get_Put(&self, out: *mut *mut HttpMethod) -> HRESULT
}}
impl ComPtr<IHttpMethodStatics> {
    #[inline] pub fn get_delete(&self) -> Result<Option<ComPtr<HttpMethod>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Delete)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_get(&self) -> Result<Option<ComPtr<HttpMethod>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Get)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_head(&self) -> Result<Option<ComPtr<HttpMethod>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Head)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_options(&self) -> Result<Option<ComPtr<HttpMethod>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Options)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_patch(&self) -> Result<Option<ComPtr<HttpMethod>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Patch)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_post(&self) -> Result<Option<ComPtr<HttpMethod>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Post)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_put(&self) -> Result<Option<ComPtr<HttpMethod>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Put)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMultipartContent, 3750849279, 39206, 19145, 170, 241, 224, 208, 78, 240, 155, 185);
RT_INTERFACE!{interface IHttpMultipartContent(IHttpMultipartContentVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMultipartContent] {
    fn Add(&self, content: *mut IHttpContent) -> HRESULT
}}
impl ComPtr<IHttpMultipartContent> {
    #[inline] pub fn add(&self, content: &ComPtr<IHttpContent>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Add)(self.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpMultipartContent: IHttpContent}
impl RtActivatable<IHttpMultipartContentFactory> for HttpMultipartContent {}
impl RtActivatable<IActivationFactory> for HttpMultipartContent {}
impl HttpMultipartContent {
    #[inline] pub fn create_with_subtype(subtype: &HStringArg) -> Result<ComPtr<HttpMultipartContent>> {
        <Self as RtActivatable<IHttpMultipartContentFactory>>::get_activation_factory().create_with_subtype(subtype)
    }
    #[inline] pub fn create_with_subtype_and_boundary(subtype: &HStringArg, boundary: &HStringArg) -> Result<ComPtr<HttpMultipartContent>> {
        <Self as RtActivatable<IHttpMultipartContentFactory>>::get_activation_factory().create_with_subtype_and_boundary(subtype, boundary)
    }
}
DEFINE_CLSID!(HttpMultipartContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,77,117,108,116,105,112,97,114,116,67,111,110,116,101,110,116,0]) [CLSID_HttpMultipartContent]);
DEFINE_IID!(IID_IHttpMultipartContentFactory, 2125737570, 546, 20256, 179, 114, 71, 213, 219, 93, 51, 180);
RT_INTERFACE!{static interface IHttpMultipartContentFactory(IHttpMultipartContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMultipartContentFactory] {
    fn CreateWithSubtype(&self, subtype: HSTRING, out: *mut *mut HttpMultipartContent) -> HRESULT,
    fn CreateWithSubtypeAndBoundary(&self, subtype: HSTRING, boundary: HSTRING, out: *mut *mut HttpMultipartContent) -> HRESULT
}}
impl ComPtr<IHttpMultipartContentFactory> {
    #[inline] pub fn create_with_subtype(&self, subtype: &HStringArg) -> Result<ComPtr<HttpMultipartContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateWithSubtype)(self.deref() as *const _ as *mut _, subtype.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_subtype_and_boundary(&self, subtype: &HStringArg, boundary: &HStringArg) -> Result<ComPtr<HttpMultipartContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateWithSubtypeAndBoundary)(self.deref() as *const _ as *mut _, subtype.get(), boundary.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMultipartFormDataContent, 1691564002, 59751, 17956, 182, 209, 207, 116, 96, 74, 74, 66);
RT_INTERFACE!{interface IHttpMultipartFormDataContent(IHttpMultipartFormDataContentVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMultipartFormDataContent] {
    fn Add(&self, content: *mut IHttpContent) -> HRESULT,
    fn AddWithName(&self, content: *mut IHttpContent, name: HSTRING) -> HRESULT,
    fn AddWithNameAndFileName(&self, content: *mut IHttpContent, name: HSTRING, fileName: HSTRING) -> HRESULT
}}
impl ComPtr<IHttpMultipartFormDataContent> {
    #[inline] pub fn add(&self, content: &ComPtr<IHttpContent>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Add)(self.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_with_name(&self, content: &ComPtr<IHttpContent>, name: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).AddWithName)(self.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _, name.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_with_name_and_file_name(&self, content: &ComPtr<IHttpContent>, name: &HStringArg, fileName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).AddWithNameAndFileName)(self.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _, name.get(), fileName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpMultipartFormDataContent: IHttpContent}
impl RtActivatable<IHttpMultipartFormDataContentFactory> for HttpMultipartFormDataContent {}
impl RtActivatable<IActivationFactory> for HttpMultipartFormDataContent {}
impl HttpMultipartFormDataContent {
    #[inline] pub fn create_with_boundary(boundary: &HStringArg) -> Result<ComPtr<HttpMultipartFormDataContent>> {
        <Self as RtActivatable<IHttpMultipartFormDataContentFactory>>::get_activation_factory().create_with_boundary(boundary)
    }
}
DEFINE_CLSID!(HttpMultipartFormDataContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,77,117,108,116,105,112,97,114,116,70,111,114,109,68,97,116,97,67,111,110,116,101,110,116,0]) [CLSID_HttpMultipartFormDataContent]);
DEFINE_IID!(IID_IHttpMultipartFormDataContentFactory, 2689430289, 20503, 17954, 147, 168, 73, 178, 74, 79, 203, 252);
RT_INTERFACE!{static interface IHttpMultipartFormDataContentFactory(IHttpMultipartFormDataContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMultipartFormDataContentFactory] {
    fn CreateWithBoundary(&self, boundary: HSTRING, out: *mut *mut HttpMultipartFormDataContent) -> HRESULT
}}
impl ComPtr<IHttpMultipartFormDataContentFactory> {
    #[inline] pub fn create_with_boundary(&self, boundary: &HStringArg) -> Result<ComPtr<HttpMultipartFormDataContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateWithBoundary)(self.deref() as *const _ as *mut _, boundary.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct HttpProgress {
    Stage: HttpProgressStage, BytesSent: u64, TotalBytesToSend: *mut foundation::IReference<u64>, BytesReceived: u64, TotalBytesToReceive: *mut foundation::IReference<u64>, Retries: u32,
}}
RT_ENUM! { enum HttpProgressStage: i32 {
    None = 0, DetectingProxy = 10, ResolvingName = 20, ConnectingToServer = 30, NegotiatingSsl = 40, SendingHeaders = 50, SendingContent = 60, WaitingForResponse = 70, ReceivingHeaders = 80, ReceivingContent = 90,
}}
DEFINE_IID!(IID_IHttpRequestMessage, 4118162236, 29908, 18449, 181, 220, 159, 139, 78, 47, 154, 191);
RT_INTERFACE!{interface IHttpRequestMessage(IHttpRequestMessageVtbl): IInspectable(IInspectableVtbl) [IID_IHttpRequestMessage] {
    fn get_Content(&self, out: *mut *mut IHttpContent) -> HRESULT,
    fn put_Content(&self, value: *mut IHttpContent) -> HRESULT,
    fn get_Headers(&self, out: *mut *mut headers::HttpRequestHeaderCollection) -> HRESULT,
    fn get_Method(&self, out: *mut *mut HttpMethod) -> HRESULT,
    fn put_Method(&self, value: *mut HttpMethod) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut foundation::collections::IMap<HString, IInspectable>) -> HRESULT,
    fn get_RequestUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_RequestUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_TransportInformation(&self, out: *mut *mut HttpTransportInformation) -> HRESULT
}}
impl ComPtr<IHttpRequestMessage> {
    #[inline] pub fn get_content(&self) -> Result<Option<ComPtr<IHttpContent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Content)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content(&self, value: &ComPtr<IHttpContent>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Content)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_headers(&self) -> Result<Option<ComPtr<headers::HttpRequestHeaderCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Headers)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_method(&self) -> Result<Option<ComPtr<HttpMethod>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Method)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_method(&self, value: &ComPtr<HttpMethod>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Method)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<ComPtr<foundation::collections::IMap<HString, IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Properties)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_request_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RequestUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_request_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_RequestUri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_transport_information(&self) -> Result<Option<ComPtr<HttpTransportInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_TransportInformation)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpRequestMessage: IHttpRequestMessage}
impl RtActivatable<IHttpRequestMessageFactory> for HttpRequestMessage {}
impl RtActivatable<IActivationFactory> for HttpRequestMessage {}
impl HttpRequestMessage {
    #[inline] pub fn create(method: &ComPtr<HttpMethod>, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<HttpRequestMessage>> {
        <Self as RtActivatable<IHttpRequestMessageFactory>>::get_activation_factory().create(method, uri)
    }
}
DEFINE_CLSID!(HttpRequestMessage(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,82,101,113,117,101,115,116,77,101,115,115,97,103,101,0]) [CLSID_HttpRequestMessage]);
DEFINE_IID!(IID_IHttpRequestMessageFactory, 1538038094, 14470, 16686, 174, 195, 82, 236, 127, 37, 97, 111);
RT_INTERFACE!{static interface IHttpRequestMessageFactory(IHttpRequestMessageFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpRequestMessageFactory] {
    fn Create(&self, method: *mut HttpMethod, uri: *mut foundation::Uri, out: *mut *mut HttpRequestMessage) -> HRESULT
}}
impl ComPtr<IHttpRequestMessageFactory> {
    #[inline] pub fn create(&self, method: &ComPtr<HttpMethod>, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<HttpRequestMessage>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, method.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpResponseMessage, 4276224251, 34404, 17632, 149, 217, 66, 105, 97, 153, 191, 252);
RT_INTERFACE!{interface IHttpResponseMessage(IHttpResponseMessageVtbl): IInspectable(IInspectableVtbl) [IID_IHttpResponseMessage] {
    fn get_Content(&self, out: *mut *mut IHttpContent) -> HRESULT,
    fn put_Content(&self, value: *mut IHttpContent) -> HRESULT,
    fn get_Headers(&self, out: *mut *mut headers::HttpResponseHeaderCollection) -> HRESULT,
    fn get_IsSuccessStatusCode(&self, out: *mut bool) -> HRESULT,
    fn get_ReasonPhrase(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ReasonPhrase(&self, value: HSTRING) -> HRESULT,
    fn get_RequestMessage(&self, out: *mut *mut HttpRequestMessage) -> HRESULT,
    fn put_RequestMessage(&self, value: *mut HttpRequestMessage) -> HRESULT,
    fn get_Source(&self, out: *mut HttpResponseMessageSource) -> HRESULT,
    fn put_Source(&self, value: HttpResponseMessageSource) -> HRESULT,
    fn get_StatusCode(&self, out: *mut HttpStatusCode) -> HRESULT,
    fn put_StatusCode(&self, value: HttpStatusCode) -> HRESULT,
    fn get_Version(&self, out: *mut HttpVersion) -> HRESULT,
    fn put_Version(&self, value: HttpVersion) -> HRESULT,
    fn EnsureSuccessStatusCode(&self, out: *mut *mut HttpResponseMessage) -> HRESULT
}}
impl ComPtr<IHttpResponseMessage> {
    #[inline] pub fn get_content(&self) -> Result<Option<ComPtr<IHttpContent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Content)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content(&self, value: &ComPtr<IHttpContent>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Content)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_headers(&self) -> Result<Option<ComPtr<headers::HttpResponseHeaderCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Headers)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_success_status_code(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsSuccessStatusCode)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_reason_phrase(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ReasonPhrase)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_reason_phrase(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ReasonPhrase)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_request_message(&self) -> Result<Option<ComPtr<HttpRequestMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RequestMessage)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_request_message(&self, value: &ComPtr<HttpRequestMessage>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_RequestMessage)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_source(&self) -> Result<HttpResponseMessageSource> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Source)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_source(&self, value: HttpResponseMessageSource) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Source)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status_code(&self) -> Result<HttpStatusCode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_StatusCode)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_status_code(&self, value: HttpStatusCode) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_StatusCode)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<HttpVersion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Version)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_version(&self, value: HttpVersion) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Version)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn ensure_success_status_code(&self) -> Result<Option<ComPtr<HttpResponseMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).EnsureSuccessStatusCode)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpResponseMessage: IHttpResponseMessage}
impl RtActivatable<IHttpResponseMessageFactory> for HttpResponseMessage {}
impl RtActivatable<IActivationFactory> for HttpResponseMessage {}
impl HttpResponseMessage {
    #[inline] pub fn create(statusCode: HttpStatusCode) -> Result<ComPtr<HttpResponseMessage>> {
        <Self as RtActivatable<IHttpResponseMessageFactory>>::get_activation_factory().create(statusCode)
    }
}
DEFINE_CLSID!(HttpResponseMessage(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,82,101,115,112,111,110,115,101,77,101,115,115,97,103,101,0]) [CLSID_HttpResponseMessage]);
DEFINE_IID!(IID_IHttpResponseMessageFactory, 1386786713, 61589, 17370, 182, 15, 124, 252, 43, 199, 234, 47);
RT_INTERFACE!{static interface IHttpResponseMessageFactory(IHttpResponseMessageFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpResponseMessageFactory] {
    fn Create(&self, statusCode: HttpStatusCode, out: *mut *mut HttpResponseMessage) -> HRESULT
}}
impl ComPtr<IHttpResponseMessageFactory> {
    #[inline] pub fn create(&self, statusCode: HttpStatusCode) -> Result<ComPtr<HttpResponseMessage>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, statusCode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum HttpResponseMessageSource: i32 {
    None = 0, Cache = 1, Network = 2,
}}
RT_ENUM! { enum HttpStatusCode: i32 {
    None = 0, Continue = 100, SwitchingProtocols = 101, Processing = 102, Ok = 200, Created = 201, Accepted = 202, NonAuthoritativeInformation = 203, NoContent = 204, ResetContent = 205, PartialContent = 206, MultiStatus = 207, AlreadyReported = 208, IMUsed = 226, MultipleChoices = 300, MovedPermanently = 301, Found = 302, SeeOther = 303, NotModified = 304, UseProxy = 305, TemporaryRedirect = 307, PermanentRedirect = 308, BadRequest = 400, Unauthorized = 401, PaymentRequired = 402, Forbidden = 403, NotFound = 404, MethodNotAllowed = 405, NotAcceptable = 406, ProxyAuthenticationRequired = 407, RequestTimeout = 408, Conflict = 409, Gone = 410, LengthRequired = 411, PreconditionFailed = 412, RequestEntityTooLarge = 413, RequestUriTooLong = 414, UnsupportedMediaType = 415, RequestedRangeNotSatisfiable = 416, ExpectationFailed = 417, UnprocessableEntity = 422, Locked = 423, FailedDependency = 424, UpgradeRequired = 426, PreconditionRequired = 428, TooManyRequests = 429, RequestHeaderFieldsTooLarge = 431, InternalServerError = 500, NotImplemented = 501, BadGateway = 502, ServiceUnavailable = 503, GatewayTimeout = 504, HttpVersionNotSupported = 505, VariantAlsoNegotiates = 506, InsufficientStorage = 507, LoopDetected = 508, NotExtended = 510, NetworkAuthenticationRequired = 511,
}}
RT_CLASS!{class HttpStreamContent: IHttpContent}
impl RtActivatable<IHttpStreamContentFactory> for HttpStreamContent {}
impl HttpStreamContent {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_input_stream(content: &ComPtr<super::super::storage::streams::IInputStream>) -> Result<ComPtr<HttpStreamContent>> {
        <Self as RtActivatable<IHttpStreamContentFactory>>::get_activation_factory().create_from_input_stream(content)
    }
}
DEFINE_CLSID!(HttpStreamContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,83,116,114,101,97,109,67,111,110,116,101,110,116,0]) [CLSID_HttpStreamContent]);
DEFINE_IID!(IID_IHttpStreamContentFactory, 4091956637, 63269, 16510, 148, 47, 14, 218, 24, 152, 9, 244);
RT_INTERFACE!{static interface IHttpStreamContentFactory(IHttpStreamContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpStreamContentFactory] {
    #[cfg(feature="windows-storage")] fn CreateFromInputStream(&self, content: *mut super::super::storage::streams::IInputStream, out: *mut *mut HttpStreamContent) -> HRESULT
}}
impl ComPtr<IHttpStreamContentFactory> {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_input_stream(&self, content: &ComPtr<super::super::storage::streams::IInputStream>) -> Result<ComPtr<HttpStreamContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromInputStream)(self.deref() as *const _ as *mut _, content.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpStringContent: IHttpContent}
impl RtActivatable<IHttpStringContentFactory> for HttpStringContent {}
impl HttpStringContent {
    #[inline] pub fn create_from_string(content: &HStringArg) -> Result<ComPtr<HttpStringContent>> {
        <Self as RtActivatable<IHttpStringContentFactory>>::get_activation_factory().create_from_string(content)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_string_with_encoding(content: &HStringArg, encoding: super::super::storage::streams::UnicodeEncoding) -> Result<ComPtr<HttpStringContent>> {
        <Self as RtActivatable<IHttpStringContentFactory>>::get_activation_factory().create_from_string_with_encoding(content, encoding)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_string_with_encoding_and_media_type(content: &HStringArg, encoding: super::super::storage::streams::UnicodeEncoding, mediaType: &HStringArg) -> Result<ComPtr<HttpStringContent>> {
        <Self as RtActivatable<IHttpStringContentFactory>>::get_activation_factory().create_from_string_with_encoding_and_media_type(content, encoding, mediaType)
    }
}
DEFINE_CLSID!(HttpStringContent(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,116,116,112,83,116,114,105,110,103,67,111,110,116,101,110,116,0]) [CLSID_HttpStringContent]);
DEFINE_IID!(IID_IHttpStringContentFactory, 1180999003, 11923, 18667, 142, 97, 25, 103, 120, 120, 229, 127);
RT_INTERFACE!{static interface IHttpStringContentFactory(IHttpStringContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpStringContentFactory] {
    fn CreateFromString(&self, content: HSTRING, out: *mut *mut HttpStringContent) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateFromStringWithEncoding(&self, content: HSTRING, encoding: super::super::storage::streams::UnicodeEncoding, out: *mut *mut HttpStringContent) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateFromStringWithEncodingAndMediaType(&self, content: HSTRING, encoding: super::super::storage::streams::UnicodeEncoding, mediaType: HSTRING, out: *mut *mut HttpStringContent) -> HRESULT
}}
impl ComPtr<IHttpStringContentFactory> {
    #[inline] pub fn create_from_string(&self, content: &HStringArg) -> Result<ComPtr<HttpStringContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromString)(self.deref() as *const _ as *mut _, content.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_string_with_encoding(&self, content: &HStringArg, encoding: super::super::storage::streams::UnicodeEncoding) -> Result<ComPtr<HttpStringContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromStringWithEncoding)(self.deref() as *const _ as *mut _, content.get(), encoding, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_string_with_encoding_and_media_type(&self, content: &HStringArg, encoding: super::super::storage::streams::UnicodeEncoding, mediaType: &HStringArg) -> Result<ComPtr<HttpStringContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromStringWithEncodingAndMediaType)(self.deref() as *const _ as *mut _, content.get(), encoding, mediaType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpTransportInformation, 1880256920, 50855, 20176, 131, 58, 131, 253, 139, 143, 23, 141);
RT_INTERFACE!{interface IHttpTransportInformation(IHttpTransportInformationVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransportInformation] {
    #[cfg(feature="windows-security")] fn get_ServerCertificate(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-networking")] fn get_ServerCertificateErrorSeverity(&self, out: *mut super::super::networking::sockets::SocketSslErrorSeverity) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerCertificateErrors(&self, out: *mut *mut foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerIntermediateCertificates(&self, out: *mut *mut foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>) -> HRESULT
}}
impl ComPtr<IHttpTransportInformation> {
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate(&self) -> Result<Option<ComPtr<super::super::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ServerCertificate)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn get_server_certificate_error_severity(&self) -> Result<super::super::networking::sockets::SocketSslErrorSeverity> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ServerCertificateErrorSeverity)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate_errors(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ServerCertificateErrors)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_intermediate_certificates(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ServerIntermediateCertificates)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpTransportInformation: IHttpTransportInformation}
RT_ENUM! { enum HttpVersion: i32 {
    None = 0, Http10 = 1, Http11 = 2, Http20 = 3,
}}
pub mod diagnostics { // Windows.Web.Http.Diagnostics
use crate::prelude::*;
DEFINE_IID!(IID_IHttpDiagnosticProvider, 3179353345, 41046, 19769, 177, 116, 131, 59, 123, 3, 176, 44);
RT_INTERFACE!{interface IHttpDiagnosticProvider(IHttpDiagnosticProviderVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProvider] {
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_RequestSent(&self, handler: *mut foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RequestSent(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ResponseReceived(&self, handler: *mut foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ResponseReceived(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RequestResponseCompleted(&self, handler: *mut foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RequestResponseCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<IHttpDiagnosticProvider> {
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Start)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Stop)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_request_sent(&self, handler: &ComPtr<foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestSentEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_RequestSent)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_request_sent(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_RequestSent)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_response_received(&self, handler: &ComPtr<foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderResponseReceivedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_ResponseReceived)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_response_received(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_ResponseReceived)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_request_response_completed(&self, handler: &ComPtr<foundation::TypedEventHandler<HttpDiagnosticProvider, HttpDiagnosticProviderRequestResponseCompletedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_RequestResponseCompleted)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_request_response_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_RequestResponseCompleted)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpDiagnosticProvider: IHttpDiagnosticProvider}
impl RtActivatable<IHttpDiagnosticProviderStatics> for HttpDiagnosticProvider {}
impl HttpDiagnosticProvider {
    #[cfg(feature="windows-system")] #[inline] pub fn create_from_process_diagnostic_info(processDiagnosticInfo: &ComPtr<crate::windows::system::diagnostics::ProcessDiagnosticInfo>) -> Result<Option<ComPtr<HttpDiagnosticProvider>>> {
        <Self as RtActivatable<IHttpDiagnosticProviderStatics>>::get_activation_factory().create_from_process_diagnostic_info(processDiagnosticInfo)
    }
}
DEFINE_CLSID!(HttpDiagnosticProvider(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,68,105,97,103,110,111,115,116,105,99,115,46,72,116,116,112,68,105,97,103,110,111,115,116,105,99,80,114,111,118,105,100,101,114,0]) [CLSID_HttpDiagnosticProvider]);
DEFINE_IID!(IID_IHttpDiagnosticProviderRequestResponseCompletedEventArgs, 1935644910, 38134, 17714, 178, 110, 97, 225, 177, 228, 239, 212);
RT_INTERFACE!{interface IHttpDiagnosticProviderRequestResponseCompletedEventArgs(IHttpDiagnosticProviderRequestResponseCompletedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderRequestResponseCompletedEventArgs] {
    fn get_ActivityId(&self, out: *mut Guid) -> HRESULT,
    fn get_Timestamps(&self, out: *mut *mut HttpDiagnosticProviderRequestResponseTimestamps) -> HRESULT,
    fn get_RequestedUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_ProcessId(&self, out: *mut u32) -> HRESULT,
    fn get_ThreadId(&self, out: *mut u32) -> HRESULT,
    fn get_Initiator(&self, out: *mut HttpDiagnosticRequestInitiator) -> HRESULT,
    fn get_SourceLocations(&self, out: *mut *mut foundation::collections::IVectorView<HttpDiagnosticSourceLocation>) -> HRESULT
}}
impl ComPtr<IHttpDiagnosticProviderRequestResponseCompletedEventArgs> {
    #[inline] pub fn get_activity_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ActivityId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_timestamps(&self) -> Result<Option<ComPtr<HttpDiagnosticProviderRequestResponseTimestamps>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Timestamps)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_requested_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RequestedUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_process_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ProcessId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_thread_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ThreadId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_initiator(&self) -> Result<HttpDiagnosticRequestInitiator> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Initiator)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_source_locations(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HttpDiagnosticSourceLocation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_SourceLocations)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpDiagnosticProviderRequestResponseCompletedEventArgs: IHttpDiagnosticProviderRequestResponseCompletedEventArgs}
DEFINE_IID!(IID_IHttpDiagnosticProviderRequestResponseTimestamps, 3769622032, 21967, 19457, 145, 212, 162, 5, 87, 216, 73, 240);
RT_INTERFACE!{interface IHttpDiagnosticProviderRequestResponseTimestamps(IHttpDiagnosticProviderRequestResponseTimestampsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderRequestResponseTimestamps] {
    fn get_CacheCheckedTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_ConnectionInitiatedTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_NameResolvedTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_SslNegotiatedTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_ConnectionCompletedTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_RequestSentTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_RequestCompletedTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_ResponseReceivedTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_ResponseCompletedTimestamp(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT
}}
impl ComPtr<IHttpDiagnosticProviderRequestResponseTimestamps> {
    #[inline] pub fn get_cache_checked_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CacheCheckedTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_connection_initiated_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ConnectionInitiatedTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name_resolved_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_NameResolvedTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ssl_negotiated_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_SslNegotiatedTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_connection_completed_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ConnectionCompletedTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_request_sent_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RequestSentTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_request_completed_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RequestCompletedTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_response_received_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ResponseReceivedTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_response_completed_timestamp(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ResponseCompletedTimestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpDiagnosticProviderRequestResponseTimestamps: IHttpDiagnosticProviderRequestResponseTimestamps}
DEFINE_IID!(IID_IHttpDiagnosticProviderRequestSentEventArgs, 1062311632, 19487, 20158, 165, 122, 6, 147, 7, 113, 197, 13);
RT_INTERFACE!{interface IHttpDiagnosticProviderRequestSentEventArgs(IHttpDiagnosticProviderRequestSentEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderRequestSentEventArgs] {
    fn get_Timestamp(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_ActivityId(&self, out: *mut Guid) -> HRESULT,
    fn get_Message(&self, out: *mut *mut super::HttpRequestMessage) -> HRESULT,
    fn get_ProcessId(&self, out: *mut u32) -> HRESULT,
    fn get_ThreadId(&self, out: *mut u32) -> HRESULT,
    fn get_Initiator(&self, out: *mut HttpDiagnosticRequestInitiator) -> HRESULT,
    fn get_SourceLocations(&self, out: *mut *mut foundation::collections::IVectorView<HttpDiagnosticSourceLocation>) -> HRESULT
}}
impl ComPtr<IHttpDiagnosticProviderRequestSentEventArgs> {
    #[inline] pub fn get_timestamp(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Timestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_activity_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ActivityId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_message(&self) -> Result<Option<ComPtr<super::HttpRequestMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Message)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_process_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ProcessId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_thread_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ThreadId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_initiator(&self) -> Result<HttpDiagnosticRequestInitiator> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Initiator)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_source_locations(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HttpDiagnosticSourceLocation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_SourceLocations)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpDiagnosticProviderRequestSentEventArgs: IHttpDiagnosticProviderRequestSentEventArgs}
DEFINE_IID!(IID_IHttpDiagnosticProviderResponseReceivedEventArgs, 2694993516, 43871, 19814, 187, 45, 8, 76, 244, 22, 53, 208);
RT_INTERFACE!{interface IHttpDiagnosticProviderResponseReceivedEventArgs(IHttpDiagnosticProviderResponseReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderResponseReceivedEventArgs] {
    fn get_Timestamp(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_ActivityId(&self, out: *mut Guid) -> HRESULT,
    fn get_Message(&self, out: *mut *mut super::HttpResponseMessage) -> HRESULT
}}
impl ComPtr<IHttpDiagnosticProviderResponseReceivedEventArgs> {
    #[inline] pub fn get_timestamp(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Timestamp)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_activity_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ActivityId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_message(&self) -> Result<Option<ComPtr<super::HttpResponseMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Message)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpDiagnosticProviderResponseReceivedEventArgs: IHttpDiagnosticProviderResponseReceivedEventArgs}
DEFINE_IID!(IID_IHttpDiagnosticProviderStatics, 1535266497, 27244, 18380, 175, 236, 30, 134, 188, 38, 5, 59);
RT_INTERFACE!{static interface IHttpDiagnosticProviderStatics(IHttpDiagnosticProviderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticProviderStatics] {
    #[cfg(feature="windows-system")] fn CreateFromProcessDiagnosticInfo(&self, processDiagnosticInfo: *mut crate::windows::system::diagnostics::ProcessDiagnosticInfo, out: *mut *mut HttpDiagnosticProvider) -> HRESULT
}}
impl ComPtr<IHttpDiagnosticProviderStatics> {
    #[cfg(feature="windows-system")] #[inline] pub fn create_from_process_diagnostic_info(&self, processDiagnosticInfo: &ComPtr<crate::windows::system::diagnostics::ProcessDiagnosticInfo>) -> Result<Option<ComPtr<HttpDiagnosticProvider>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromProcessDiagnosticInfo)(self.deref() as *const _ as *mut _, processDiagnosticInfo.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum HttpDiagnosticRequestInitiator: i32 {
    ParsedElement = 0, Script = 1, Image = 2, Link = 3, Style = 4, XmlHttpRequest = 5, Media = 6, HtmlDownload = 7, Prefetch = 8, Other = 9, CrossOriginPreFlight = 10, Fetch = 11, Beacon = 12,
}}
DEFINE_IID!(IID_IHttpDiagnosticSourceLocation, 1420415584, 34912, 16959, 182, 250, 215, 119, 22, 246, 71, 167);
RT_INTERFACE!{interface IHttpDiagnosticSourceLocation(IHttpDiagnosticSourceLocationVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDiagnosticSourceLocation] {
    fn get_SourceUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_LineNumber(&self, out: *mut u64) -> HRESULT,
    fn get_ColumnNumber(&self, out: *mut u64) -> HRESULT
}}
impl ComPtr<IHttpDiagnosticSourceLocation> {
    #[inline] pub fn get_source_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_SourceUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_line_number(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_LineNumber)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_column_number(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ColumnNumber)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpDiagnosticSourceLocation: IHttpDiagnosticSourceLocation}
} // Windows.Web.Http.Diagnostics
pub mod filters { // Windows.Web.Http.Filters
use crate::prelude::*;
DEFINE_IID!(IID_IHttpBaseProtocolFilter, 1908972297, 57649, 19284, 165, 60, 235, 67, 255, 55, 233, 187);
RT_INTERFACE!{interface IHttpBaseProtocolFilter(IHttpBaseProtocolFilterVtbl): IInspectable(IInspectableVtbl) [IID_IHttpBaseProtocolFilter] {
    fn get_AllowAutoRedirect(&self, out: *mut bool) -> HRESULT,
    fn put_AllowAutoRedirect(&self, value: bool) -> HRESULT,
    fn get_AllowUI(&self, out: *mut bool) -> HRESULT,
    fn put_AllowUI(&self, value: bool) -> HRESULT,
    fn get_AutomaticDecompression(&self, out: *mut bool) -> HRESULT,
    fn put_AutomaticDecompression(&self, value: bool) -> HRESULT,
    fn get_CacheControl(&self, out: *mut *mut HttpCacheControl) -> HRESULT,
    fn get_CookieManager(&self, out: *mut *mut super::HttpCookieManager) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ClientCertificate(&self, out: *mut *mut crate::windows::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy9(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ClientCertificate(&self, value: *mut crate::windows::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy10(&self) -> (),
    #[cfg(feature="windows-security")] fn get_IgnorableServerCertificateErrors(&self, out: *mut *mut foundation::collections::IVector<crate::windows::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    fn get_MaxConnectionsPerServer(&self, out: *mut u32) -> HRESULT,
    fn put_MaxConnectionsPerServer(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ProxyCredential(&self, out: *mut *mut crate::windows::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy14(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ProxyCredential(&self, value: *mut crate::windows::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy15(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCredential(&self, out: *mut *mut crate::windows::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy16(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ServerCredential(&self, value: *mut crate::windows::security::credentials::PasswordCredential) -> HRESULT,
    fn get_UseProxy(&self, out: *mut bool) -> HRESULT,
    fn put_UseProxy(&self, value: bool) -> HRESULT
}}
impl ComPtr<IHttpBaseProtocolFilter> {
    #[inline] pub fn get_allow_auto_redirect(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_AllowAutoRedirect)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_auto_redirect(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_AllowAutoRedirect)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_ui(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_AllowUI)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_ui(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_AllowUI)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_automatic_decompression(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_AutomaticDecompression)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_automatic_decompression(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_AutomaticDecompression)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cache_control(&self) -> Result<Option<ComPtr<HttpCacheControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CacheControl)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cookie_manager(&self) -> Result<Option<ComPtr<super::HttpCookieManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CookieManager)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_client_certificate(&self) -> Result<Option<ComPtr<crate::windows::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ClientCertificate)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_client_certificate(&self, value: &ComPtr<crate::windows::security::cryptography::certificates::Certificate>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ClientCertificate)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_ignorable_server_certificate_errors(&self) -> Result<Option<ComPtr<foundation::collections::IVector<crate::windows::security::cryptography::certificates::ChainValidationResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_IgnorableServerCertificateErrors)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_max_connections_per_server(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_MaxConnectionsPerServer)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_connections_per_server(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MaxConnectionsPerServer)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_proxy_credential(&self) -> Result<Option<ComPtr<crate::windows::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ProxyCredential)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_proxy_credential(&self, value: &ComPtr<crate::windows::security::credentials::PasswordCredential>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ProxyCredential)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_credential(&self) -> Result<Option<ComPtr<crate::windows::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ServerCredential)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_server_credential(&self, value: &ComPtr<crate::windows::security::credentials::PasswordCredential>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ServerCredential)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_use_proxy(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_UseProxy)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_use_proxy(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_UseProxy)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpBaseProtocolFilter: IHttpBaseProtocolFilter}
impl RtActivatable<IActivationFactory> for HttpBaseProtocolFilter {}
DEFINE_CLSID!(HttpBaseProtocolFilter(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,70,105,108,116,101,114,115,46,72,116,116,112,66,97,115,101,80,114,111,116,111,99,111,108,70,105,108,116,101,114,0]) [CLSID_HttpBaseProtocolFilter]);
DEFINE_IID!(IID_IHttpBaseProtocolFilter2, 784531475, 37927, 18688, 160, 23, 250, 125, 163, 181, 201, 174);
RT_INTERFACE!{interface IHttpBaseProtocolFilter2(IHttpBaseProtocolFilter2Vtbl): IInspectable(IInspectableVtbl) [IID_IHttpBaseProtocolFilter2] {
    fn get_MaxVersion(&self, out: *mut super::HttpVersion) -> HRESULT,
    fn put_MaxVersion(&self, value: super::HttpVersion) -> HRESULT
}}
impl ComPtr<IHttpBaseProtocolFilter2> {
    #[inline] pub fn get_max_version(&self) -> Result<super::HttpVersion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_MaxVersion)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_version(&self, value: super::HttpVersion) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MaxVersion)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpBaseProtocolFilter3, 3560918348, 48450, 17326, 135, 23, 173, 44, 143, 75, 41, 55);
RT_INTERFACE!{interface IHttpBaseProtocolFilter3(IHttpBaseProtocolFilter3Vtbl): IInspectable(IInspectableVtbl) [IID_IHttpBaseProtocolFilter3] {
    fn get_CookieUsageBehavior(&self, out: *mut HttpCookieUsageBehavior) -> HRESULT,
    fn put_CookieUsageBehavior(&self, value: HttpCookieUsageBehavior) -> HRESULT
}}
impl ComPtr<IHttpBaseProtocolFilter3> {
    #[inline] pub fn get_cookie_usage_behavior(&self) -> Result<HttpCookieUsageBehavior> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_CookieUsageBehavior)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_cookie_usage_behavior(&self, value: HttpCookieUsageBehavior) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_CookieUsageBehavior)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpBaseProtocolFilter4, 2682481871, 10627, 18579, 148, 31, 235, 81, 140, 168, 206, 249);
RT_INTERFACE!{interface IHttpBaseProtocolFilter4(IHttpBaseProtocolFilter4Vtbl): IInspectable(IInspectableVtbl) [IID_IHttpBaseProtocolFilter4] {
    fn add_ServerCustomValidationRequested(&self, eventHandler: *mut foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ServerCustomValidationRequested(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn ClearAuthenticationCache(&self) -> HRESULT
}}
impl ComPtr<IHttpBaseProtocolFilter4> {
    #[inline] pub fn add_server_custom_validation_requested(&self, eventHandler: &ComPtr<foundation::TypedEventHandler<HttpBaseProtocolFilter, HttpServerCustomValidationRequestedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_ServerCustomValidationRequested)(self.deref() as *const _ as *mut _, eventHandler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_server_custom_validation_requested(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_ServerCustomValidationRequested)(self.deref() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear_authentication_cache(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ClearAuthenticationCache)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpCacheControl, 3346930868, 15594, 20149, 172, 133, 4, 225, 134, 230, 58, 183);
RT_INTERFACE!{interface IHttpCacheControl(IHttpCacheControlVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCacheControl] {
    fn get_ReadBehavior(&self, out: *mut HttpCacheReadBehavior) -> HRESULT,
    fn put_ReadBehavior(&self, value: HttpCacheReadBehavior) -> HRESULT,
    fn get_WriteBehavior(&self, out: *mut HttpCacheWriteBehavior) -> HRESULT,
    fn put_WriteBehavior(&self, value: HttpCacheWriteBehavior) -> HRESULT
}}
impl ComPtr<IHttpCacheControl> {
    #[inline] pub fn get_read_behavior(&self) -> Result<HttpCacheReadBehavior> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ReadBehavior)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_read_behavior(&self, value: HttpCacheReadBehavior) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ReadBehavior)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_write_behavior(&self) -> Result<HttpCacheWriteBehavior> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_WriteBehavior)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_write_behavior(&self, value: HttpCacheWriteBehavior) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_WriteBehavior)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpCacheControl: IHttpCacheControl}
RT_ENUM! { enum HttpCacheReadBehavior: i32 {
    Default = 0, MostRecent = 1, OnlyFromCache = 2, NoCache = 3,
}}
RT_ENUM! { enum HttpCacheWriteBehavior: i32 {
    Default = 0, NoCache = 1,
}}
RT_ENUM! { enum HttpCookieUsageBehavior: i32 {
    Default = 0, NoCookies = 1,
}}
DEFINE_IID!(IID_IHttpFilter, 2764795349, 2306, 17310, 191, 215, 225, 37, 82, 177, 101, 206);
RT_INTERFACE!{interface IHttpFilter(IHttpFilterVtbl): IInspectable(IInspectableVtbl) [IID_IHttpFilter] {
    fn SendRequestAsync(&self, request: *mut super::HttpRequestMessage, out: *mut *mut foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>) -> HRESULT
}}
impl ComPtr<IHttpFilter> {
    #[inline] pub fn send_request_async(&self, request: &ComPtr<super::HttpRequestMessage>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<super::HttpResponseMessage, super::HttpProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).SendRequestAsync)(self.deref() as *const _ as *mut _, request.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpServerCustomValidationRequestedEventArgs, 828767794, 59357, 18615, 163, 97, 147, 156, 117, 14, 99, 204);
RT_INTERFACE!{interface IHttpServerCustomValidationRequestedEventArgs(IHttpServerCustomValidationRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpServerCustomValidationRequestedEventArgs] {
    fn get_RequestMessage(&self, out: *mut *mut super::HttpRequestMessage) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCertificate(&self, out: *mut *mut crate::windows::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-networking")] fn get_ServerCertificateErrorSeverity(&self, out: *mut crate::windows::networking::sockets::SocketSslErrorSeverity) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCertificateErrors(&self, out: *mut *mut foundation::collections::IVectorView<crate::windows::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerIntermediateCertificates(&self, out: *mut *mut foundation::collections::IVectorView<crate::windows::security::cryptography::certificates::Certificate>) -> HRESULT,
    fn Reject(&self) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl ComPtr<IHttpServerCustomValidationRequestedEventArgs> {
    #[inline] pub fn get_request_message(&self) -> Result<Option<ComPtr<super::HttpRequestMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RequestMessage)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate(&self) -> Result<Option<ComPtr<crate::windows::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ServerCertificate)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn get_server_certificate_error_severity(&self) -> Result<crate::windows::networking::sockets::SocketSslErrorSeverity> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ServerCertificateErrorSeverity)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate_errors(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<crate::windows::security::cryptography::certificates::ChainValidationResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ServerCertificateErrors)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_intermediate_certificates(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<crate::windows::security::cryptography::certificates::Certificate>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ServerIntermediateCertificates)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn reject(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Reject)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetDeferral)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpServerCustomValidationRequestedEventArgs: IHttpServerCustomValidationRequestedEventArgs}
} // Windows.Web.Http.Filters
pub mod headers { // Windows.Web.Http.Headers
use crate::prelude::*;
DEFINE_IID!(IID_IHttpCacheDirectiveHeaderValueCollection, 2589485961, 54736, 20414, 189, 157, 181, 179, 99, 104, 17, 180);
RT_INTERFACE!{interface IHttpCacheDirectiveHeaderValueCollection(IHttpCacheDirectiveHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCacheDirectiveHeaderValueCollection] {
    fn get_MaxAge(&self, out: *mut *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn put_MaxAge(&self, value: *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn get_MaxStale(&self, out: *mut *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn put_MaxStale(&self, value: *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn get_MinFresh(&self, out: *mut *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn put_MinFresh(&self, value: *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn get_SharedMaxAge(&self, out: *mut *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn put_SharedMaxAge(&self, value: *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpCacheDirectiveHeaderValueCollection> {
    #[inline] pub fn get_max_age(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::TimeSpan>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MaxAge)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_max_age(&self, value: &ComPtr<foundation::IReference<foundation::TimeSpan>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MaxAge)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_stale(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::TimeSpan>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MaxStale)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_max_stale(&self, value: &ComPtr<foundation::IReference<foundation::TimeSpan>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MaxStale)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_min_fresh(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::TimeSpan>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MinFresh)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_min_fresh(&self, value: &ComPtr<foundation::IReference<foundation::TimeSpan>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MinFresh)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_shared_max_age(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::TimeSpan>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_SharedMaxAge)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_shared_max_age(&self, value: &ComPtr<foundation::IReference<foundation::TimeSpan>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_SharedMaxAge)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpCacheDirectiveHeaderValueCollection: IHttpCacheDirectiveHeaderValueCollection}
DEFINE_IID!(IID_IHttpChallengeHeaderValue, 959668655, 3965, 18464, 159, 221, 162, 185, 86, 238, 174, 171);
RT_INTERFACE!{interface IHttpChallengeHeaderValue(IHttpChallengeHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpChallengeHeaderValue] {
    fn get_Parameters(&self, out: *mut *mut foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Scheme(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Token(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHttpChallengeHeaderValue> {
    #[inline] pub fn get_parameters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HttpNameValueHeaderValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Parameters)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scheme(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Scheme)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_token(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Token)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpChallengeHeaderValue: IHttpChallengeHeaderValue}
impl RtActivatable<IHttpChallengeHeaderValueFactory> for HttpChallengeHeaderValue {}
impl RtActivatable<IHttpChallengeHeaderValueStatics> for HttpChallengeHeaderValue {}
impl HttpChallengeHeaderValue {
    #[inline] pub fn create_from_scheme(scheme: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> {
        <Self as RtActivatable<IHttpChallengeHeaderValueFactory>>::get_activation_factory().create_from_scheme(scheme)
    }
    #[inline] pub fn create_from_scheme_with_token(scheme: &HStringArg, token: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> {
        <Self as RtActivatable<IHttpChallengeHeaderValueFactory>>::get_activation_factory().create_from_scheme_with_token(scheme, token)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpChallengeHeaderValue>>> {
        <Self as RtActivatable<IHttpChallengeHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpChallengeHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpChallengeHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpChallengeHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,104,97,108,108,101,110,103,101,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpChallengeHeaderValue]);
DEFINE_IID!(IID_IHttpChallengeHeaderValueCollection, 3399376769, 44768, 17235, 161, 11, 230, 37, 186, 189, 100, 194);
RT_INTERFACE!{interface IHttpChallengeHeaderValueCollection(IHttpChallengeHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpChallengeHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpChallengeHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpChallengeHeaderValueCollection: IHttpChallengeHeaderValueCollection}
DEFINE_IID!(IID_IHttpChallengeHeaderValueFactory, 3293758545, 55708, 16554, 147, 153, 144, 238, 185, 143, 198, 19);
RT_INTERFACE!{static interface IHttpChallengeHeaderValueFactory(IHttpChallengeHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpChallengeHeaderValueFactory] {
    fn CreateFromScheme(&self, scheme: HSTRING, out: *mut *mut HttpChallengeHeaderValue) -> HRESULT,
    fn CreateFromSchemeWithToken(&self, scheme: HSTRING, token: HSTRING, out: *mut *mut HttpChallengeHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpChallengeHeaderValueFactory> {
    #[inline] pub fn create_from_scheme(&self, scheme: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromScheme)(self.deref() as *const _ as *mut _, scheme.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_scheme_with_token(&self, scheme: &HStringArg, token: &HStringArg) -> Result<ComPtr<HttpChallengeHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromSchemeWithToken)(self.deref() as *const _ as *mut _, scheme.get(), token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpChallengeHeaderValueStatics, 4090727026, 64513, 19713, 160, 8, 252, 183, 196, 89, 214, 53);
RT_INTERFACE!{static interface IHttpChallengeHeaderValueStatics(IHttpChallengeHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpChallengeHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpChallengeHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, challengeHeaderValue: *mut *mut HttpChallengeHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpChallengeHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpChallengeHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpChallengeHeaderValue>>, bool)> { unsafe { 
        let mut challengeHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut challengeHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(challengeHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpConnectionOptionHeaderValue, 3410686586, 20112, 17899, 141, 205, 253, 20, 8, 244, 196, 79);
RT_INTERFACE!{interface IHttpConnectionOptionHeaderValue(IHttpConnectionOptionHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpConnectionOptionHeaderValue] {
    fn get_Token(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHttpConnectionOptionHeaderValue> {
    #[inline] pub fn get_token(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Token)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpConnectionOptionHeaderValue: IHttpConnectionOptionHeaderValue}
impl RtActivatable<IHttpConnectionOptionHeaderValueFactory> for HttpConnectionOptionHeaderValue {}
impl RtActivatable<IHttpConnectionOptionHeaderValueStatics> for HttpConnectionOptionHeaderValue {}
impl HttpConnectionOptionHeaderValue {
    #[inline] pub fn create(token: &HStringArg) -> Result<ComPtr<HttpConnectionOptionHeaderValue>> {
        <Self as RtActivatable<IHttpConnectionOptionHeaderValueFactory>>::get_activation_factory().create(token)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpConnectionOptionHeaderValue>>> {
        <Self as RtActivatable<IHttpConnectionOptionHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpConnectionOptionHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpConnectionOptionHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpConnectionOptionHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,110,101,99,116,105,111,110,79,112,116,105,111,110,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpConnectionOptionHeaderValue]);
DEFINE_IID!(IID_IHttpConnectionOptionHeaderValueCollection, 3841289245, 20802, 19968, 142, 15, 1, 149, 9, 51, 118, 41);
RT_INTERFACE!{interface IHttpConnectionOptionHeaderValueCollection(IHttpConnectionOptionHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpConnectionOptionHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpConnectionOptionHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpConnectionOptionHeaderValueCollection: IHttpConnectionOptionHeaderValueCollection}
DEFINE_IID!(IID_IHttpConnectionOptionHeaderValueFactory, 3644640286, 2941, 19519, 165, 141, 162, 161, 189, 234, 188, 10);
RT_INTERFACE!{static interface IHttpConnectionOptionHeaderValueFactory(IHttpConnectionOptionHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpConnectionOptionHeaderValueFactory] {
    fn Create(&self, token: HSTRING, out: *mut *mut HttpConnectionOptionHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpConnectionOptionHeaderValueFactory> {
    #[inline] pub fn create(&self, token: &HStringArg) -> Result<ComPtr<HttpConnectionOptionHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpConnectionOptionHeaderValueStatics, 2863095095, 43334, 19231, 133, 175, 72, 182, 139, 60, 80, 189);
RT_INTERFACE!{static interface IHttpConnectionOptionHeaderValueStatics(IHttpConnectionOptionHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpConnectionOptionHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpConnectionOptionHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, connectionOptionHeaderValue: *mut *mut HttpConnectionOptionHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpConnectionOptionHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpConnectionOptionHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpConnectionOptionHeaderValue>>, bool)> { unsafe { 
        let mut connectionOptionHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut connectionOptionHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(connectionOptionHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpContentCodingHeaderValue, 3170367786, 37750, 19845, 188, 204, 159, 79, 154, 202, 180, 52);
RT_INTERFACE!{interface IHttpContentCodingHeaderValue(IHttpContentCodingHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingHeaderValue] {
    fn get_ContentCoding(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHttpContentCodingHeaderValue> {
    #[inline] pub fn get_content_coding(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentCoding)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpContentCodingHeaderValue: IHttpContentCodingHeaderValue}
impl RtActivatable<IHttpContentCodingHeaderValueFactory> for HttpContentCodingHeaderValue {}
impl RtActivatable<IHttpContentCodingHeaderValueStatics> for HttpContentCodingHeaderValue {}
impl HttpContentCodingHeaderValue {
    #[inline] pub fn create(contentCoding: &HStringArg) -> Result<ComPtr<HttpContentCodingHeaderValue>> {
        <Self as RtActivatable<IHttpContentCodingHeaderValueFactory>>::get_activation_factory().create(contentCoding)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpContentCodingHeaderValue>>> {
        <Self as RtActivatable<IHttpContentCodingHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpContentCodingHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpContentCodingHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpContentCodingHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,67,111,100,105,110,103,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpContentCodingHeaderValue]);
DEFINE_IID!(IID_IHttpContentCodingHeaderValueCollection, 2099386145, 42715, 17262, 142, 131, 145, 89, 97, 146, 129, 156);
RT_INTERFACE!{interface IHttpContentCodingHeaderValueCollection(IHttpContentCodingHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpContentCodingHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpContentCodingHeaderValueCollection: IHttpContentCodingHeaderValueCollection}
DEFINE_IID!(IID_IHttpContentCodingHeaderValueFactory, 3309120471, 13099, 17232, 133, 16, 46, 103, 162, 40, 154, 90);
RT_INTERFACE!{static interface IHttpContentCodingHeaderValueFactory(IHttpContentCodingHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingHeaderValueFactory] {
    fn Create(&self, contentCoding: HSTRING, out: *mut *mut HttpContentCodingHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpContentCodingHeaderValueFactory> {
    #[inline] pub fn create(&self, contentCoding: &HStringArg) -> Result<ComPtr<HttpContentCodingHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, contentCoding.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpContentCodingHeaderValueStatics, 2497208366, 63935, 17143, 170, 70, 237, 39, 42, 65, 226, 18);
RT_INTERFACE!{static interface IHttpContentCodingHeaderValueStatics(IHttpContentCodingHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpContentCodingHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, contentCodingHeaderValue: *mut *mut HttpContentCodingHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpContentCodingHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpContentCodingHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpContentCodingHeaderValue>>, bool)> { unsafe { 
        let mut contentCodingHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut contentCodingHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(contentCodingHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpContentCodingWithQualityHeaderValue, 2488474837, 35603, 19827, 134, 81, 247, 107, 56, 248, 132, 149);
RT_INTERFACE!{interface IHttpContentCodingWithQualityHeaderValue(IHttpContentCodingWithQualityHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingWithQualityHeaderValue] {
    fn get_ContentCoding(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Quality(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT
}}
impl ComPtr<IHttpContentCodingWithQualityHeaderValue> {
    #[inline] pub fn get_content_coding(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentCoding)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_quality(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Quality)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpContentCodingWithQualityHeaderValue: IHttpContentCodingWithQualityHeaderValue}
impl RtActivatable<IHttpContentCodingWithQualityHeaderValueFactory> for HttpContentCodingWithQualityHeaderValue {}
impl RtActivatable<IHttpContentCodingWithQualityHeaderValueStatics> for HttpContentCodingWithQualityHeaderValue {}
impl HttpContentCodingWithQualityHeaderValue {
    #[inline] pub fn create_from_value(contentCoding: &HStringArg) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> {
        <Self as RtActivatable<IHttpContentCodingWithQualityHeaderValueFactory>>::get_activation_factory().create_from_value(contentCoding)
    }
    #[inline] pub fn create_from_value_with_quality(contentCoding: &HStringArg, quality: f64) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> {
        <Self as RtActivatable<IHttpContentCodingWithQualityHeaderValueFactory>>::get_activation_factory().create_from_value_with_quality(contentCoding, quality)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpContentCodingWithQualityHeaderValue>>> {
        <Self as RtActivatable<IHttpContentCodingWithQualityHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpContentCodingWithQualityHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpContentCodingWithQualityHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpContentCodingWithQualityHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,67,111,100,105,110,103,87,105,116,104,81,117,97,108,105,116,121,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpContentCodingWithQualityHeaderValue]);
DEFINE_IID!(IID_IHttpContentCodingWithQualityHeaderValueCollection, 2081256766, 59545, 17272, 181, 200, 65, 45, 130, 7, 17, 204);
RT_INTERFACE!{interface IHttpContentCodingWithQualityHeaderValueCollection(IHttpContentCodingWithQualityHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingWithQualityHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpContentCodingWithQualityHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpContentCodingWithQualityHeaderValueCollection: IHttpContentCodingWithQualityHeaderValueCollection}
DEFINE_IID!(IID_IHttpContentCodingWithQualityHeaderValueFactory, 3294555674, 50515, 18172, 173, 226, 215, 92, 29, 83, 223, 123);
RT_INTERFACE!{static interface IHttpContentCodingWithQualityHeaderValueFactory(IHttpContentCodingWithQualityHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingWithQualityHeaderValueFactory] {
    fn CreateFromValue(&self, contentCoding: HSTRING, out: *mut *mut HttpContentCodingWithQualityHeaderValue) -> HRESULT,
    fn CreateFromValueWithQuality(&self, contentCoding: HSTRING, quality: f64, out: *mut *mut HttpContentCodingWithQualityHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpContentCodingWithQualityHeaderValueFactory> {
    #[inline] pub fn create_from_value(&self, contentCoding: &HStringArg) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromValue)(self.deref() as *const _ as *mut _, contentCoding.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_value_with_quality(&self, contentCoding: &HStringArg, quality: f64) -> Result<ComPtr<HttpContentCodingWithQualityHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromValueWithQuality)(self.deref() as *const _ as *mut _, contentCoding.get(), quality, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpContentCodingWithQualityHeaderValueStatics, 3905500540, 36745, 18433, 142, 117, 76, 154, 191, 195, 222, 113);
RT_INTERFACE!{static interface IHttpContentCodingWithQualityHeaderValueStatics(IHttpContentCodingWithQualityHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentCodingWithQualityHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpContentCodingWithQualityHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, contentCodingWithQualityHeaderValue: *mut *mut HttpContentCodingWithQualityHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpContentCodingWithQualityHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpContentCodingWithQualityHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpContentCodingWithQualityHeaderValue>>, bool)> { unsafe { 
        let mut contentCodingWithQualityHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut contentCodingWithQualityHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(contentCodingWithQualityHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpContentDispositionHeaderValue, 4070764252, 9769, 19273, 153, 8, 150, 161, 104, 233, 54, 94);
RT_INTERFACE!{interface IHttpContentDispositionHeaderValue(IHttpContentDispositionHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentDispositionHeaderValue] {
    fn get_DispositionType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DispositionType(&self, value: HSTRING) -> HRESULT,
    fn get_FileName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FileName(&self, value: HSTRING) -> HRESULT,
    fn get_FileNameStar(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FileNameStar(&self, value: HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_Parameters(&self, out: *mut *mut foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Size(&self, out: *mut *mut foundation::IReference<u64>) -> HRESULT,
    fn put_Size(&self, value: *mut foundation::IReference<u64>) -> HRESULT
}}
impl ComPtr<IHttpContentDispositionHeaderValue> {
    #[inline] pub fn get_disposition_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_DispositionType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_disposition_type(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_DispositionType)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_file_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_FileName)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_file_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_FileName)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_file_name_star(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_FileNameStar)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_file_name_star(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_FileNameStar)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Name)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_parameters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HttpNameValueHeaderValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Parameters)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<Option<ComPtr<foundation::IReference<u64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Size)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_size(&self, value: &ComPtr<foundation::IReference<u64>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Size)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpContentDispositionHeaderValue: IHttpContentDispositionHeaderValue}
impl RtActivatable<IHttpContentDispositionHeaderValueFactory> for HttpContentDispositionHeaderValue {}
impl RtActivatable<IHttpContentDispositionHeaderValueStatics> for HttpContentDispositionHeaderValue {}
impl HttpContentDispositionHeaderValue {
    #[inline] pub fn create(dispositionType: &HStringArg) -> Result<ComPtr<HttpContentDispositionHeaderValue>> {
        <Self as RtActivatable<IHttpContentDispositionHeaderValueFactory>>::get_activation_factory().create(dispositionType)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpContentDispositionHeaderValue>>> {
        <Self as RtActivatable<IHttpContentDispositionHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpContentDispositionHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpContentDispositionHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpContentDispositionHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,68,105,115,112,111,115,105,116,105,111,110,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpContentDispositionHeaderValue]);
DEFINE_IID!(IID_IHttpContentDispositionHeaderValueFactory, 2568338372, 17772, 20097, 130, 149, 178, 171, 60, 188, 245, 69);
RT_INTERFACE!{static interface IHttpContentDispositionHeaderValueFactory(IHttpContentDispositionHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentDispositionHeaderValueFactory] {
    fn Create(&self, dispositionType: HSTRING, out: *mut *mut HttpContentDispositionHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpContentDispositionHeaderValueFactory> {
    #[inline] pub fn create(&self, dispositionType: &HStringArg) -> Result<ComPtr<HttpContentDispositionHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, dispositionType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpContentDispositionHeaderValueStatics, 700801127, 23095, 18148, 176, 116, 197, 23, 125, 105, 202, 102);
RT_INTERFACE!{static interface IHttpContentDispositionHeaderValueStatics(IHttpContentDispositionHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentDispositionHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpContentDispositionHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, contentDispositionHeaderValue: *mut *mut HttpContentDispositionHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpContentDispositionHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpContentDispositionHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpContentDispositionHeaderValue>>, bool)> { unsafe { 
        let mut contentDispositionHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut contentDispositionHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(contentDispositionHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpContentHeaderCollection, 1080109636, 18350, 19326, 145, 36, 105, 98, 139, 100, 170, 24);
RT_INTERFACE!{interface IHttpContentHeaderCollection(IHttpContentHeaderCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentHeaderCollection] {
    fn get_ContentDisposition(&self, out: *mut *mut HttpContentDispositionHeaderValue) -> HRESULT,
    fn put_ContentDisposition(&self, value: *mut HttpContentDispositionHeaderValue) -> HRESULT,
    fn get_ContentEncoding(&self, out: *mut *mut HttpContentCodingHeaderValueCollection) -> HRESULT,
    fn get_ContentLanguage(&self, out: *mut *mut HttpLanguageHeaderValueCollection) -> HRESULT,
    fn get_ContentLength(&self, out: *mut *mut foundation::IReference<u64>) -> HRESULT,
    fn put_ContentLength(&self, value: *mut foundation::IReference<u64>) -> HRESULT,
    fn get_ContentLocation(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_ContentLocation(&self, value: *mut foundation::Uri) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ContentMD5(&self, out: *mut *mut crate::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy9(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_ContentMD5(&self, value: *mut crate::windows::storage::streams::IBuffer) -> HRESULT,
    fn get_ContentRange(&self, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn put_ContentRange(&self, value: *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn get_ContentType(&self, out: *mut *mut HttpMediaTypeHeaderValue) -> HRESULT,
    fn put_ContentType(&self, value: *mut HttpMediaTypeHeaderValue) -> HRESULT,
    fn get_Expires(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn put_Expires(&self, value: *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_LastModified(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn put_LastModified(&self, value: *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn Append(&self, name: HSTRING, value: HSTRING) -> HRESULT,
    fn TryAppendWithoutValidation(&self, name: HSTRING, value: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpContentHeaderCollection> {
    #[inline] pub fn get_content_disposition(&self) -> Result<Option<ComPtr<HttpContentDispositionHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentDisposition)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_disposition(&self, value: &ComPtr<HttpContentDispositionHeaderValue>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ContentDisposition)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_content_encoding(&self) -> Result<Option<ComPtr<HttpContentCodingHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentEncoding)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_content_language(&self) -> Result<Option<ComPtr<HttpLanguageHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentLanguage)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_content_length(&self) -> Result<Option<ComPtr<foundation::IReference<u64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentLength)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_length(&self, value: &ComPtr<foundation::IReference<u64>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ContentLength)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_content_location(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentLocation)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_location(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ContentLocation)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_content_md5(&self) -> Result<Option<ComPtr<crate::windows::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentMD5)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_content_md5(&self, value: &ComPtr<crate::windows::storage::streams::IBuffer>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ContentMD5)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_content_range(&self) -> Result<Option<ComPtr<HttpContentRangeHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentRange)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_range(&self, value: &ComPtr<HttpContentRangeHeaderValue>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ContentRange)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_content_type(&self) -> Result<Option<ComPtr<HttpMediaTypeHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ContentType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_type(&self, value: &ComPtr<HttpMediaTypeHeaderValue>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ContentType)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_expires(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Expires)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_expires(&self, value: &ComPtr<foundation::IReference<foundation::DateTime>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Expires)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_last_modified(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_LastModified)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_last_modified(&self, value: &ComPtr<foundation::IReference<foundation::DateTime>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_LastModified)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn append(&self, name: &HStringArg, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Append)(self.deref() as *const _ as *mut _, name.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_append_without_validation(&self, name: &HStringArg, value: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryAppendWithoutValidation)(self.deref() as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpContentHeaderCollection: IHttpContentHeaderCollection}
impl RtActivatable<IActivationFactory> for HttpContentHeaderCollection {}
DEFINE_CLSID!(HttpContentHeaderCollection(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,72,101,97,100,101,114,67,111,108,108,101,99,116,105,111,110,0]) [CLSID_HttpContentHeaderCollection]);
DEFINE_IID!(IID_IHttpContentRangeHeaderValue, 81356755, 42230, 18780, 149, 48, 133, 121, 252, 186, 138, 169);
RT_INTERFACE!{interface IHttpContentRangeHeaderValue(IHttpContentRangeHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentRangeHeaderValue] {
    fn get_FirstBytePosition(&self, out: *mut *mut foundation::IReference<u64>) -> HRESULT,
    fn get_LastBytePosition(&self, out: *mut *mut foundation::IReference<u64>) -> HRESULT,
    fn get_Length(&self, out: *mut *mut foundation::IReference<u64>) -> HRESULT,
    fn get_Unit(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Unit(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<IHttpContentRangeHeaderValue> {
    #[inline] pub fn get_first_byte_position(&self) -> Result<Option<ComPtr<foundation::IReference<u64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_FirstBytePosition)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_byte_position(&self) -> Result<Option<ComPtr<foundation::IReference<u64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_LastBytePosition)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_length(&self) -> Result<Option<ComPtr<foundation::IReference<u64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Length)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_unit(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Unit)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_unit(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Unit)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpContentRangeHeaderValue: IHttpContentRangeHeaderValue}
impl RtActivatable<IHttpContentRangeHeaderValueFactory> for HttpContentRangeHeaderValue {}
impl RtActivatable<IHttpContentRangeHeaderValueStatics> for HttpContentRangeHeaderValue {}
impl HttpContentRangeHeaderValue {
    #[inline] pub fn create_from_length(length: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> {
        <Self as RtActivatable<IHttpContentRangeHeaderValueFactory>>::get_activation_factory().create_from_length(length)
    }
    #[inline] pub fn create_from_range(from: u64, to: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> {
        <Self as RtActivatable<IHttpContentRangeHeaderValueFactory>>::get_activation_factory().create_from_range(from, to)
    }
    #[inline] pub fn create_from_range_with_length(from: u64, to: u64, length: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> {
        <Self as RtActivatable<IHttpContentRangeHeaderValueFactory>>::get_activation_factory().create_from_range_with_length(from, to, length)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpContentRangeHeaderValue>>> {
        <Self as RtActivatable<IHttpContentRangeHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpContentRangeHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpContentRangeHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpContentRangeHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,110,116,101,110,116,82,97,110,103,101,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpContentRangeHeaderValue]);
DEFINE_IID!(IID_IHttpContentRangeHeaderValueFactory, 1062983313, 41020, 17494, 154, 111, 239, 39, 236, 208, 60, 174);
RT_INTERFACE!{static interface IHttpContentRangeHeaderValueFactory(IHttpContentRangeHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentRangeHeaderValueFactory] {
    fn CreateFromLength(&self, length: u64, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn CreateFromRange(&self, from: u64, to: u64, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn CreateFromRangeWithLength(&self, from: u64, to: u64, length: u64, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpContentRangeHeaderValueFactory> {
    #[inline] pub fn create_from_length(&self, length: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromLength)(self.deref() as *const _ as *mut _, length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_range(&self, from: u64, to: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromRange)(self.deref() as *const _ as *mut _, from, to, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_range_with_length(&self, from: u64, to: u64, length: u64) -> Result<ComPtr<HttpContentRangeHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromRangeWithLength)(self.deref() as *const _ as *mut _, from, to, length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpContentRangeHeaderValueStatics, 2158184138, 5964, 20398, 130, 28, 19, 76, 210, 148, 170, 56);
RT_INTERFACE!{static interface IHttpContentRangeHeaderValueStatics(IHttpContentRangeHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpContentRangeHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpContentRangeHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, contentRangeHeaderValue: *mut *mut HttpContentRangeHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpContentRangeHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpContentRangeHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpContentRangeHeaderValue>>, bool)> { unsafe { 
        let mut contentRangeHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut contentRangeHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(contentRangeHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpCookiePairHeaderValue, 3419693591, 19241, 16683, 189, 144, 179, 216, 20, 171, 142, 27);
RT_INTERFACE!{interface IHttpCookiePairHeaderValue(IHttpCookiePairHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookiePairHeaderValue] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<IHttpCookiePairHeaderValue> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Value)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpCookiePairHeaderValue: IHttpCookiePairHeaderValue}
impl RtActivatable<IHttpCookiePairHeaderValueFactory> for HttpCookiePairHeaderValue {}
impl RtActivatable<IHttpCookiePairHeaderValueStatics> for HttpCookiePairHeaderValue {}
impl HttpCookiePairHeaderValue {
    #[inline] pub fn create_from_name(name: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> {
        <Self as RtActivatable<IHttpCookiePairHeaderValueFactory>>::get_activation_factory().create_from_name(name)
    }
    #[inline] pub fn create_from_name_with_value(name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> {
        <Self as RtActivatable<IHttpCookiePairHeaderValueFactory>>::get_activation_factory().create_from_name_with_value(name, value)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpCookiePairHeaderValue>>> {
        <Self as RtActivatable<IHttpCookiePairHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpCookiePairHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpCookiePairHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpCookiePairHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,111,111,107,105,101,80,97,105,114,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpCookiePairHeaderValue]);
DEFINE_IID!(IID_IHttpCookiePairHeaderValueCollection, 4092871504, 22558, 20172, 159, 89, 229, 7, 208, 79, 6, 230);
RT_INTERFACE!{interface IHttpCookiePairHeaderValueCollection(IHttpCookiePairHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookiePairHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpCookiePairHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpCookiePairHeaderValueCollection: IHttpCookiePairHeaderValueCollection}
DEFINE_IID!(IID_IHttpCookiePairHeaderValueFactory, 1667117679, 5231, 20310, 170, 33, 44, 183, 214, 213, 139, 30);
RT_INTERFACE!{static interface IHttpCookiePairHeaderValueFactory(IHttpCookiePairHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookiePairHeaderValueFactory] {
    fn CreateFromName(&self, name: HSTRING, out: *mut *mut HttpCookiePairHeaderValue) -> HRESULT,
    fn CreateFromNameWithValue(&self, name: HSTRING, value: HSTRING, out: *mut *mut HttpCookiePairHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpCookiePairHeaderValueFactory> {
    #[inline] pub fn create_from_name(&self, name: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromName)(self.deref() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_name_with_value(&self, name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpCookiePairHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromNameWithValue)(self.deref() as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpCookiePairHeaderValueStatics, 1854303560, 1711, 17506, 129, 88, 153, 56, 141, 93, 202, 129);
RT_INTERFACE!{static interface IHttpCookiePairHeaderValueStatics(IHttpCookiePairHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCookiePairHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpCookiePairHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, cookiePairHeaderValue: *mut *mut HttpCookiePairHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpCookiePairHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpCookiePairHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpCookiePairHeaderValue>>, bool)> { unsafe { 
        let mut cookiePairHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut cookiePairHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(cookiePairHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpCredentialsHeaderValue, 3276587979, 21550, 16759, 166, 199, 182, 116, 206, 25, 63, 191);
RT_INTERFACE!{interface IHttpCredentialsHeaderValue(IHttpCredentialsHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCredentialsHeaderValue] {
    fn get_Parameters(&self, out: *mut *mut foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Scheme(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Token(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHttpCredentialsHeaderValue> {
    #[inline] pub fn get_parameters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HttpNameValueHeaderValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Parameters)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scheme(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Scheme)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_token(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Token)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpCredentialsHeaderValue: IHttpCredentialsHeaderValue}
impl RtActivatable<IHttpCredentialsHeaderValueFactory> for HttpCredentialsHeaderValue {}
impl RtActivatable<IHttpCredentialsHeaderValueStatics> for HttpCredentialsHeaderValue {}
impl HttpCredentialsHeaderValue {
    #[inline] pub fn create_from_scheme(scheme: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> {
        <Self as RtActivatable<IHttpCredentialsHeaderValueFactory>>::get_activation_factory().create_from_scheme(scheme)
    }
    #[inline] pub fn create_from_scheme_with_token(scheme: &HStringArg, token: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> {
        <Self as RtActivatable<IHttpCredentialsHeaderValueFactory>>::get_activation_factory().create_from_scheme_with_token(scheme, token)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpCredentialsHeaderValue>>> {
        <Self as RtActivatable<IHttpCredentialsHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpCredentialsHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpCredentialsHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpCredentialsHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,67,114,101,100,101,110,116,105,97,108,115,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpCredentialsHeaderValue]);
DEFINE_IID!(IID_IHttpCredentialsHeaderValueFactory, 4062027409, 19740, 16770, 191, 209, 52, 71, 10, 98, 249, 80);
RT_INTERFACE!{static interface IHttpCredentialsHeaderValueFactory(IHttpCredentialsHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCredentialsHeaderValueFactory] {
    fn CreateFromScheme(&self, scheme: HSTRING, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn CreateFromSchemeWithToken(&self, scheme: HSTRING, token: HSTRING, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpCredentialsHeaderValueFactory> {
    #[inline] pub fn create_from_scheme(&self, scheme: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromScheme)(self.deref() as *const _ as *mut _, scheme.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_scheme_with_token(&self, scheme: &HStringArg, token: &HStringArg) -> Result<ComPtr<HttpCredentialsHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromSchemeWithToken)(self.deref() as *const _ as *mut _, scheme.get(), token.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpCredentialsHeaderValueStatics, 2795187174, 52876, 17475, 163, 90, 27, 114, 123, 19, 16, 54);
RT_INTERFACE!{static interface IHttpCredentialsHeaderValueStatics(IHttpCredentialsHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpCredentialsHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, credentialsHeaderValue: *mut *mut HttpCredentialsHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpCredentialsHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpCredentialsHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpCredentialsHeaderValue>>, bool)> { unsafe { 
        let mut credentialsHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut credentialsHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(credentialsHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpDateOrDeltaHeaderValue, 3942427242, 50396, 18914, 162, 125, 4, 58, 223, 88, 103, 163);
RT_INTERFACE!{interface IHttpDateOrDeltaHeaderValue(IHttpDateOrDeltaHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDateOrDeltaHeaderValue] {
    fn get_Date(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_Delta(&self, out: *mut *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT
}}
impl ComPtr<IHttpDateOrDeltaHeaderValue> {
    #[inline] pub fn get_date(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Date)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_delta(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::TimeSpan>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Delta)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpDateOrDeltaHeaderValue: IHttpDateOrDeltaHeaderValue}
impl RtActivatable<IHttpDateOrDeltaHeaderValueStatics> for HttpDateOrDeltaHeaderValue {}
impl HttpDateOrDeltaHeaderValue {
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpDateOrDeltaHeaderValue>>> {
        <Self as RtActivatable<IHttpDateOrDeltaHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpDateOrDeltaHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpDateOrDeltaHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpDateOrDeltaHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,68,97,116,101,79,114,68,101,108,116,97,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpDateOrDeltaHeaderValue]);
DEFINE_IID!(IID_IHttpDateOrDeltaHeaderValueStatics, 2082888104, 26226, 20112, 154, 154, 243, 151, 102, 247, 245, 118);
RT_INTERFACE!{static interface IHttpDateOrDeltaHeaderValueStatics(IHttpDateOrDeltaHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpDateOrDeltaHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpDateOrDeltaHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, dateOrDeltaHeaderValue: *mut *mut HttpDateOrDeltaHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpDateOrDeltaHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpDateOrDeltaHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpDateOrDeltaHeaderValue>>, bool)> { unsafe { 
        let mut dateOrDeltaHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut dateOrDeltaHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(dateOrDeltaHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpExpectationHeaderValue, 1290110413, 15001, 17327, 162, 230, 236, 35, 47, 234, 150, 88);
RT_INTERFACE!{interface IHttpExpectationHeaderValue(IHttpExpectationHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpExpectationHeaderValue] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT,
    fn get_Parameters(&self, out: *mut *mut foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT
}}
impl ComPtr<IHttpExpectationHeaderValue> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Value)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_parameters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HttpNameValueHeaderValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Parameters)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpExpectationHeaderValue: IHttpExpectationHeaderValue}
impl RtActivatable<IHttpExpectationHeaderValueFactory> for HttpExpectationHeaderValue {}
impl RtActivatable<IHttpExpectationHeaderValueStatics> for HttpExpectationHeaderValue {}
impl HttpExpectationHeaderValue {
    #[inline] pub fn create_from_name(name: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> {
        <Self as RtActivatable<IHttpExpectationHeaderValueFactory>>::get_activation_factory().create_from_name(name)
    }
    #[inline] pub fn create_from_name_with_value(name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> {
        <Self as RtActivatable<IHttpExpectationHeaderValueFactory>>::get_activation_factory().create_from_name_with_value(name, value)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpExpectationHeaderValue>>> {
        <Self as RtActivatable<IHttpExpectationHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpExpectationHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpExpectationHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpExpectationHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,69,120,112,101,99,116,97,116,105,111,110,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpExpectationHeaderValue]);
DEFINE_IID!(IID_IHttpExpectationHeaderValueCollection, 3884261811, 41186, 19140, 158, 102, 121, 112, 108, 185, 253, 88);
RT_INTERFACE!{interface IHttpExpectationHeaderValueCollection(IHttpExpectationHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpExpectationHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpExpectationHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpExpectationHeaderValueCollection: IHttpExpectationHeaderValueCollection}
DEFINE_IID!(IID_IHttpExpectationHeaderValueFactory, 1319269835, 54590, 18536, 136, 86, 30, 33, 165, 3, 13, 192);
RT_INTERFACE!{static interface IHttpExpectationHeaderValueFactory(IHttpExpectationHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpExpectationHeaderValueFactory] {
    fn CreateFromName(&self, name: HSTRING, out: *mut *mut HttpExpectationHeaderValue) -> HRESULT,
    fn CreateFromNameWithValue(&self, name: HSTRING, value: HSTRING, out: *mut *mut HttpExpectationHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpExpectationHeaderValueFactory> {
    #[inline] pub fn create_from_name(&self, name: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromName)(self.deref() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_name_with_value(&self, name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpExpectationHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromNameWithValue)(self.deref() as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpExpectationHeaderValueStatics, 806988770, 53221, 18235, 165, 127, 251, 165, 177, 78, 178, 87);
RT_INTERFACE!{static interface IHttpExpectationHeaderValueStatics(IHttpExpectationHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpExpectationHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpExpectationHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, expectationHeaderValue: *mut *mut HttpExpectationHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpExpectationHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpExpectationHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpExpectationHeaderValue>>, bool)> { unsafe { 
        let mut expectationHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut expectationHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(expectationHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpLanguageHeaderValueCollection, 2663218339, 33305, 17654, 153, 2, 140, 86, 223, 211, 52, 12);
RT_INTERFACE!{interface IHttpLanguageHeaderValueCollection(IHttpLanguageHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpLanguageHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpLanguageHeaderValueCollection: IHttpLanguageHeaderValueCollection}
DEFINE_IID!(IID_IHttpLanguageRangeWithQualityHeaderValue, 1918296322, 128, 19892, 160, 131, 125, 231, 178, 229, 186, 76);
RT_INTERFACE!{interface IHttpLanguageRangeWithQualityHeaderValue(IHttpLanguageRangeWithQualityHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageRangeWithQualityHeaderValue] {
    fn get_LanguageRange(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Quality(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT
}}
impl ComPtr<IHttpLanguageRangeWithQualityHeaderValue> {
    #[inline] pub fn get_language_range(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_LanguageRange)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_quality(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Quality)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpLanguageRangeWithQualityHeaderValue: IHttpLanguageRangeWithQualityHeaderValue}
impl RtActivatable<IHttpLanguageRangeWithQualityHeaderValueFactory> for HttpLanguageRangeWithQualityHeaderValue {}
impl RtActivatable<IHttpLanguageRangeWithQualityHeaderValueStatics> for HttpLanguageRangeWithQualityHeaderValue {}
impl HttpLanguageRangeWithQualityHeaderValue {
    #[inline] pub fn create_from_language_range(languageRange: &HStringArg) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> {
        <Self as RtActivatable<IHttpLanguageRangeWithQualityHeaderValueFactory>>::get_activation_factory().create_from_language_range(languageRange)
    }
    #[inline] pub fn create_from_language_range_with_quality(languageRange: &HStringArg, quality: f64) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> {
        <Self as RtActivatable<IHttpLanguageRangeWithQualityHeaderValueFactory>>::get_activation_factory().create_from_language_range_with_quality(languageRange, quality)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpLanguageRangeWithQualityHeaderValue>>> {
        <Self as RtActivatable<IHttpLanguageRangeWithQualityHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpLanguageRangeWithQualityHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpLanguageRangeWithQualityHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpLanguageRangeWithQualityHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,76,97,110,103,117,97,103,101,82,97,110,103,101,87,105,116,104,81,117,97,108,105,116,121,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpLanguageRangeWithQualityHeaderValue]);
DEFINE_IID!(IID_IHttpLanguageRangeWithQualityHeaderValueCollection, 2287819453, 19279, 18442, 137, 206, 138, 237, 206, 230, 227, 160);
RT_INTERFACE!{interface IHttpLanguageRangeWithQualityHeaderValueCollection(IHttpLanguageRangeWithQualityHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageRangeWithQualityHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpLanguageRangeWithQualityHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpLanguageRangeWithQualityHeaderValueCollection: IHttpLanguageRangeWithQualityHeaderValueCollection}
DEFINE_IID!(IID_IHttpLanguageRangeWithQualityHeaderValueFactory, 2075670896, 30735, 19587, 159, 228, 220, 48, 135, 246, 189, 85);
RT_INTERFACE!{static interface IHttpLanguageRangeWithQualityHeaderValueFactory(IHttpLanguageRangeWithQualityHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageRangeWithQualityHeaderValueFactory] {
    fn CreateFromLanguageRange(&self, languageRange: HSTRING, out: *mut *mut HttpLanguageRangeWithQualityHeaderValue) -> HRESULT,
    fn CreateFromLanguageRangeWithQuality(&self, languageRange: HSTRING, quality: f64, out: *mut *mut HttpLanguageRangeWithQualityHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpLanguageRangeWithQualityHeaderValueFactory> {
    #[inline] pub fn create_from_language_range(&self, languageRange: &HStringArg) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromLanguageRange)(self.deref() as *const _ as *mut _, languageRange.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_language_range_with_quality(&self, languageRange: &HStringArg, quality: f64) -> Result<ComPtr<HttpLanguageRangeWithQualityHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromLanguageRangeWithQuality)(self.deref() as *const _ as *mut _, languageRange.get(), quality, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpLanguageRangeWithQualityHeaderValueStatics, 625074502, 62216, 18165, 182, 149, 66, 245, 64, 36, 236, 104);
RT_INTERFACE!{static interface IHttpLanguageRangeWithQualityHeaderValueStatics(IHttpLanguageRangeWithQualityHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpLanguageRangeWithQualityHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpLanguageRangeWithQualityHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, languageRangeWithQualityHeaderValue: *mut *mut HttpLanguageRangeWithQualityHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpLanguageRangeWithQualityHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpLanguageRangeWithQualityHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpLanguageRangeWithQualityHeaderValue>>, bool)> { unsafe { 
        let mut languageRangeWithQualityHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut languageRangeWithQualityHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(languageRangeWithQualityHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMediaTypeHeaderValue, 380798259, 59176, 20427, 189, 176, 8, 164, 49, 161, 72, 68);
RT_INTERFACE!{interface IHttpMediaTypeHeaderValue(IHttpMediaTypeHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeHeaderValue] {
    fn get_CharSet(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CharSet(&self, value: HSTRING) -> HRESULT,
    fn get_MediaType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_MediaType(&self, value: HSTRING) -> HRESULT,
    fn get_Parameters(&self, out: *mut *mut foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT
}}
impl ComPtr<IHttpMediaTypeHeaderValue> {
    #[inline] pub fn get_char_set(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CharSet)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_char_set(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_CharSet)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MediaType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_media_type(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MediaType)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_parameters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HttpNameValueHeaderValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Parameters)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpMediaTypeHeaderValue: IHttpMediaTypeHeaderValue}
impl RtActivatable<IHttpMediaTypeHeaderValueFactory> for HttpMediaTypeHeaderValue {}
impl RtActivatable<IHttpMediaTypeHeaderValueStatics> for HttpMediaTypeHeaderValue {}
impl HttpMediaTypeHeaderValue {
    #[inline] pub fn create(mediaType: &HStringArg) -> Result<ComPtr<HttpMediaTypeHeaderValue>> {
        <Self as RtActivatable<IHttpMediaTypeHeaderValueFactory>>::get_activation_factory().create(mediaType)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpMediaTypeHeaderValue>>> {
        <Self as RtActivatable<IHttpMediaTypeHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpMediaTypeHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpMediaTypeHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpMediaTypeHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,77,101,100,105,97,84,121,112,101,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpMediaTypeHeaderValue]);
DEFINE_IID!(IID_IHttpMediaTypeHeaderValueFactory, 3201779624, 52503, 17117, 147, 103, 171, 156, 91, 86, 221, 125);
RT_INTERFACE!{static interface IHttpMediaTypeHeaderValueFactory(IHttpMediaTypeHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeHeaderValueFactory] {
    fn Create(&self, mediaType: HSTRING, out: *mut *mut HttpMediaTypeHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpMediaTypeHeaderValueFactory> {
    #[inline] pub fn create(&self, mediaType: &HStringArg) -> Result<ComPtr<HttpMediaTypeHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, mediaType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMediaTypeHeaderValueStatics, 3763176415, 7489, 19852, 162, 222, 111, 210, 237, 135, 57, 155);
RT_INTERFACE!{static interface IHttpMediaTypeHeaderValueStatics(IHttpMediaTypeHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpMediaTypeHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, mediaTypeHeaderValue: *mut *mut HttpMediaTypeHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpMediaTypeHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpMediaTypeHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpMediaTypeHeaderValue>>, bool)> { unsafe { 
        let mut mediaTypeHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut mediaTypeHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(mediaTypeHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMediaTypeWithQualityHeaderValue, 411917874, 30398, 17568, 177, 205, 32, 116, 189, 237, 45, 222);
RT_INTERFACE!{interface IHttpMediaTypeWithQualityHeaderValue(IHttpMediaTypeWithQualityHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeWithQualityHeaderValue] {
    fn get_CharSet(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CharSet(&self, value: HSTRING) -> HRESULT,
    fn get_MediaType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_MediaType(&self, value: HSTRING) -> HRESULT,
    fn get_Parameters(&self, out: *mut *mut foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Quality(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn put_Quality(&self, value: *mut foundation::IReference<f64>) -> HRESULT
}}
impl ComPtr<IHttpMediaTypeWithQualityHeaderValue> {
    #[inline] pub fn get_char_set(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CharSet)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_char_set(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_CharSet)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MediaType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_media_type(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MediaType)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_parameters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HttpNameValueHeaderValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Parameters)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_quality(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Quality)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_quality(&self, value: &ComPtr<foundation::IReference<f64>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Quality)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpMediaTypeWithQualityHeaderValue: IHttpMediaTypeWithQualityHeaderValue}
impl RtActivatable<IHttpMediaTypeWithQualityHeaderValueFactory> for HttpMediaTypeWithQualityHeaderValue {}
impl RtActivatable<IHttpMediaTypeWithQualityHeaderValueStatics> for HttpMediaTypeWithQualityHeaderValue {}
impl HttpMediaTypeWithQualityHeaderValue {
    #[inline] pub fn create_from_media_type(mediaType: &HStringArg) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> {
        <Self as RtActivatable<IHttpMediaTypeWithQualityHeaderValueFactory>>::get_activation_factory().create_from_media_type(mediaType)
    }
    #[inline] pub fn create_from_media_type_with_quality(mediaType: &HStringArg, quality: f64) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> {
        <Self as RtActivatable<IHttpMediaTypeWithQualityHeaderValueFactory>>::get_activation_factory().create_from_media_type_with_quality(mediaType, quality)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpMediaTypeWithQualityHeaderValue>>> {
        <Self as RtActivatable<IHttpMediaTypeWithQualityHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpMediaTypeWithQualityHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpMediaTypeWithQualityHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpMediaTypeWithQualityHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,77,101,100,105,97,84,121,112,101,87,105,116,104,81,117,97,108,105,116,121,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpMediaTypeWithQualityHeaderValue]);
DEFINE_IID!(IID_IHttpMediaTypeWithQualityHeaderValueCollection, 1007446899, 4930, 17799, 160, 86, 24, 208, 47, 246, 113, 101);
RT_INTERFACE!{interface IHttpMediaTypeWithQualityHeaderValueCollection(IHttpMediaTypeWithQualityHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeWithQualityHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpMediaTypeWithQualityHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpMediaTypeWithQualityHeaderValueCollection: IHttpMediaTypeWithQualityHeaderValueCollection}
DEFINE_IID!(IID_IHttpMediaTypeWithQualityHeaderValueFactory, 1282220276, 37975, 17638, 163, 35, 209, 34, 185, 88, 120, 11);
RT_INTERFACE!{static interface IHttpMediaTypeWithQualityHeaderValueFactory(IHttpMediaTypeWithQualityHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeWithQualityHeaderValueFactory] {
    fn CreateFromMediaType(&self, mediaType: HSTRING, out: *mut *mut HttpMediaTypeWithQualityHeaderValue) -> HRESULT,
    fn CreateFromMediaTypeWithQuality(&self, mediaType: HSTRING, quality: f64, out: *mut *mut HttpMediaTypeWithQualityHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpMediaTypeWithQualityHeaderValueFactory> {
    #[inline] pub fn create_from_media_type(&self, mediaType: &HStringArg) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromMediaType)(self.deref() as *const _ as *mut _, mediaType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_media_type_with_quality(&self, mediaType: &HStringArg, quality: f64) -> Result<ComPtr<HttpMediaTypeWithQualityHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromMediaTypeWithQuality)(self.deref() as *const _ as *mut _, mediaType.get(), quality, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMediaTypeWithQualityHeaderValueStatics, 1527188697, 46432, 20424, 152, 53, 126, 108, 10, 101, 123, 36);
RT_INTERFACE!{static interface IHttpMediaTypeWithQualityHeaderValueStatics(IHttpMediaTypeWithQualityHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMediaTypeWithQualityHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpMediaTypeWithQualityHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, mediaTypeWithQualityHeaderValue: *mut *mut HttpMediaTypeWithQualityHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpMediaTypeWithQualityHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpMediaTypeWithQualityHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpMediaTypeWithQualityHeaderValue>>, bool)> { unsafe { 
        let mut mediaTypeWithQualityHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut mediaTypeWithQualityHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(mediaTypeWithQualityHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpMethodHeaderValueCollection, 1136410612, 24857, 19167, 147, 140, 52, 191, 255, 207, 146, 237);
RT_INTERFACE!{interface IHttpMethodHeaderValueCollection(IHttpMethodHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpMethodHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpMethodHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpMethodHeaderValueCollection: IHttpMethodHeaderValueCollection}
DEFINE_IID!(IID_IHttpNameValueHeaderValue, 3636098147, 23450, 19739, 147, 249, 170, 91, 68, 236, 253, 223);
RT_INTERFACE!{interface IHttpNameValueHeaderValue(IHttpNameValueHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpNameValueHeaderValue] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<IHttpNameValueHeaderValue> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Value)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpNameValueHeaderValue: IHttpNameValueHeaderValue}
impl RtActivatable<IHttpNameValueHeaderValueFactory> for HttpNameValueHeaderValue {}
impl RtActivatable<IHttpNameValueHeaderValueStatics> for HttpNameValueHeaderValue {}
impl HttpNameValueHeaderValue {
    #[inline] pub fn create_from_name(name: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> {
        <Self as RtActivatable<IHttpNameValueHeaderValueFactory>>::get_activation_factory().create_from_name(name)
    }
    #[inline] pub fn create_from_name_with_value(name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> {
        <Self as RtActivatable<IHttpNameValueHeaderValueFactory>>::get_activation_factory().create_from_name_with_value(name, value)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpNameValueHeaderValue>>> {
        <Self as RtActivatable<IHttpNameValueHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpNameValueHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpNameValueHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpNameValueHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,78,97,109,101,86,97,108,117,101,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpNameValueHeaderValue]);
DEFINE_IID!(IID_IHttpNameValueHeaderValueFactory, 1997415015, 52216, 18230, 169, 37, 147, 251, 225, 12, 124, 168);
RT_INTERFACE!{static interface IHttpNameValueHeaderValueFactory(IHttpNameValueHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpNameValueHeaderValueFactory] {
    fn CreateFromName(&self, name: HSTRING, out: *mut *mut HttpNameValueHeaderValue) -> HRESULT,
    fn CreateFromNameWithValue(&self, name: HSTRING, value: HSTRING, out: *mut *mut HttpNameValueHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpNameValueHeaderValueFactory> {
    #[inline] pub fn create_from_name(&self, name: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromName)(self.deref() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_name_with_value(&self, name: &HStringArg, value: &HStringArg) -> Result<ComPtr<HttpNameValueHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromNameWithValue)(self.deref() as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpNameValueHeaderValueStatics, 4292084495, 4400, 16722, 134, 89, 37, 105, 9, 169, 209, 21);
RT_INTERFACE!{static interface IHttpNameValueHeaderValueStatics(IHttpNameValueHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpNameValueHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpNameValueHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, nameValueHeaderValue: *mut *mut HttpNameValueHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpNameValueHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpNameValueHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpNameValueHeaderValue>>, bool)> { unsafe { 
        let mut nameValueHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut nameValueHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(nameValueHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpProductHeaderValue, 4110347779, 60372, 16736, 185, 255, 128, 124, 81, 131, 182, 230);
RT_INTERFACE!{interface IHttpProductHeaderValue(IHttpProductHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductHeaderValue] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Version(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHttpProductHeaderValue> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Version)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpProductHeaderValue: IHttpProductHeaderValue}
impl RtActivatable<IHttpProductHeaderValueFactory> for HttpProductHeaderValue {}
impl RtActivatable<IHttpProductHeaderValueStatics> for HttpProductHeaderValue {}
impl HttpProductHeaderValue {
    #[inline] pub fn create_from_name(productName: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> {
        <Self as RtActivatable<IHttpProductHeaderValueFactory>>::get_activation_factory().create_from_name(productName)
    }
    #[inline] pub fn create_from_name_with_version(productName: &HStringArg, productVersion: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> {
        <Self as RtActivatable<IHttpProductHeaderValueFactory>>::get_activation_factory().create_from_name_with_version(productName, productVersion)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpProductHeaderValue>>> {
        <Self as RtActivatable<IHttpProductHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpProductHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpProductHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpProductHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,80,114,111,100,117,99,116,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpProductHeaderValue]);
DEFINE_IID!(IID_IHttpProductHeaderValueFactory, 1629136117, 33468, 17147, 151, 123, 220, 0, 83, 110, 94, 134);
RT_INTERFACE!{static interface IHttpProductHeaderValueFactory(IHttpProductHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductHeaderValueFactory] {
    fn CreateFromName(&self, productName: HSTRING, out: *mut *mut HttpProductHeaderValue) -> HRESULT,
    fn CreateFromNameWithVersion(&self, productName: HSTRING, productVersion: HSTRING, out: *mut *mut HttpProductHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpProductHeaderValueFactory> {
    #[inline] pub fn create_from_name(&self, productName: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromName)(self.deref() as *const _ as *mut _, productName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_name_with_version(&self, productName: &HStringArg, productVersion: &HStringArg) -> Result<ComPtr<HttpProductHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromNameWithVersion)(self.deref() as *const _ as *mut _, productName.get(), productVersion.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpProductHeaderValueStatics, 2428714537, 48892, 17207, 190, 98, 73, 240, 151, 151, 95, 83);
RT_INTERFACE!{static interface IHttpProductHeaderValueStatics(IHttpProductHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpProductHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, productHeaderValue: *mut *mut HttpProductHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpProductHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpProductHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpProductHeaderValue>>, bool)> { unsafe { 
        let mut productHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut productHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(productHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpProductInfoHeaderValue, 454723378, 19509, 18538, 150, 111, 100, 100, 137, 25, 142, 77);
RT_INTERFACE!{interface IHttpProductInfoHeaderValue(IHttpProductInfoHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductInfoHeaderValue] {
    fn get_Product(&self, out: *mut *mut HttpProductHeaderValue) -> HRESULT,
    fn get_Comment(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHttpProductInfoHeaderValue> {
    #[inline] pub fn get_product(&self) -> Result<Option<ComPtr<HttpProductHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Product)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_comment(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Comment)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpProductInfoHeaderValue: IHttpProductInfoHeaderValue}
impl RtActivatable<IHttpProductInfoHeaderValueFactory> for HttpProductInfoHeaderValue {}
impl RtActivatable<IHttpProductInfoHeaderValueStatics> for HttpProductInfoHeaderValue {}
impl HttpProductInfoHeaderValue {
    #[inline] pub fn create_from_comment(productComment: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> {
        <Self as RtActivatable<IHttpProductInfoHeaderValueFactory>>::get_activation_factory().create_from_comment(productComment)
    }
    #[inline] pub fn create_from_name_with_version(productName: &HStringArg, productVersion: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> {
        <Self as RtActivatable<IHttpProductInfoHeaderValueFactory>>::get_activation_factory().create_from_name_with_version(productName, productVersion)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpProductInfoHeaderValue>>> {
        <Self as RtActivatable<IHttpProductInfoHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpProductInfoHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpProductInfoHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpProductInfoHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,80,114,111,100,117,99,116,73,110,102,111,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpProductInfoHeaderValue]);
DEFINE_IID!(IID_IHttpProductInfoHeaderValueCollection, 2273179466, 54939, 17656, 173, 79, 69, 58, 249, 196, 46, 208);
RT_INTERFACE!{interface IHttpProductInfoHeaderValueCollection(IHttpProductInfoHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductInfoHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpProductInfoHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpProductInfoHeaderValueCollection: IHttpProductInfoHeaderValueCollection}
DEFINE_IID!(IID_IHttpProductInfoHeaderValueFactory, 606212030, 60094, 17508, 180, 96, 236, 1, 11, 124, 65, 226);
RT_INTERFACE!{static interface IHttpProductInfoHeaderValueFactory(IHttpProductInfoHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductInfoHeaderValueFactory] {
    fn CreateFromComment(&self, productComment: HSTRING, out: *mut *mut HttpProductInfoHeaderValue) -> HRESULT,
    fn CreateFromNameWithVersion(&self, productName: HSTRING, productVersion: HSTRING, out: *mut *mut HttpProductInfoHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpProductInfoHeaderValueFactory> {
    #[inline] pub fn create_from_comment(&self, productComment: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromComment)(self.deref() as *const _ as *mut _, productComment.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_name_with_version(&self, productName: &HStringArg, productVersion: &HStringArg) -> Result<ComPtr<HttpProductInfoHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateFromNameWithVersion)(self.deref() as *const _ as *mut _, productName.get(), productVersion.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpProductInfoHeaderValueStatics, 3682588759, 12922, 20083, 129, 229, 112, 89, 163, 2, 176, 66);
RT_INTERFACE!{static interface IHttpProductInfoHeaderValueStatics(IHttpProductInfoHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpProductInfoHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpProductInfoHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, productInfoHeaderValue: *mut *mut HttpProductInfoHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpProductInfoHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpProductInfoHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpProductInfoHeaderValue>>, bool)> { unsafe { 
        let mut productInfoHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut productInfoHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(productInfoHeaderValue), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpRequestHeaderCollection, 2940220059, 46404, 18075, 134, 185, 172, 61, 70, 111, 234, 54);
RT_INTERFACE!{interface IHttpRequestHeaderCollection(IHttpRequestHeaderCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpRequestHeaderCollection] {
    fn get_Accept(&self, out: *mut *mut HttpMediaTypeWithQualityHeaderValueCollection) -> HRESULT,
    fn get_AcceptEncoding(&self, out: *mut *mut HttpContentCodingWithQualityHeaderValueCollection) -> HRESULT,
    fn get_AcceptLanguage(&self, out: *mut *mut HttpLanguageRangeWithQualityHeaderValueCollection) -> HRESULT,
    fn get_Authorization(&self, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn put_Authorization(&self, value: *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn get_CacheControl(&self, out: *mut *mut HttpCacheDirectiveHeaderValueCollection) -> HRESULT,
    fn get_Connection(&self, out: *mut *mut HttpConnectionOptionHeaderValueCollection) -> HRESULT,
    fn get_Cookie(&self, out: *mut *mut HttpCookiePairHeaderValueCollection) -> HRESULT,
    fn get_Date(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn put_Date(&self, value: *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_Expect(&self, out: *mut *mut HttpExpectationHeaderValueCollection) -> HRESULT,
    fn get_From(&self, out: *mut HSTRING) -> HRESULT,
    fn put_From(&self, value: HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-networking")] fn get_Host(&self, out: *mut *mut crate::windows::networking::HostName) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy14(&self) -> (),
    #[cfg(feature="windows-networking")] fn put_Host(&self, value: *mut crate::windows::networking::HostName) -> HRESULT,
    fn get_IfModifiedSince(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn put_IfModifiedSince(&self, value: *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_IfUnmodifiedSince(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn put_IfUnmodifiedSince(&self, value: *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_MaxForwards(&self, out: *mut *mut foundation::IReference<u32>) -> HRESULT,
    fn put_MaxForwards(&self, value: *mut foundation::IReference<u32>) -> HRESULT,
    fn get_ProxyAuthorization(&self, out: *mut *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn put_ProxyAuthorization(&self, value: *mut HttpCredentialsHeaderValue) -> HRESULT,
    fn get_Referer(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_Referer(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_TransferEncoding(&self, out: *mut *mut HttpTransferCodingHeaderValueCollection) -> HRESULT,
    fn get_UserAgent(&self, out: *mut *mut HttpProductInfoHeaderValueCollection) -> HRESULT,
    fn Append(&self, name: HSTRING, value: HSTRING) -> HRESULT,
    fn TryAppendWithoutValidation(&self, name: HSTRING, value: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpRequestHeaderCollection> {
    #[inline] pub fn get_accept(&self) -> Result<Option<ComPtr<HttpMediaTypeWithQualityHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Accept)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_accept_encoding(&self) -> Result<Option<ComPtr<HttpContentCodingWithQualityHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_AcceptEncoding)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_accept_language(&self) -> Result<Option<ComPtr<HttpLanguageRangeWithQualityHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_AcceptLanguage)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_authorization(&self) -> Result<Option<ComPtr<HttpCredentialsHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Authorization)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_authorization(&self, value: &ComPtr<HttpCredentialsHeaderValue>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Authorization)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cache_control(&self) -> Result<Option<ComPtr<HttpCacheDirectiveHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CacheControl)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_connection(&self) -> Result<Option<ComPtr<HttpConnectionOptionHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Connection)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cookie(&self) -> Result<Option<ComPtr<HttpCookiePairHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Cookie)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_date(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Date)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_date(&self, value: &ComPtr<foundation::IReference<foundation::DateTime>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Date)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_expect(&self) -> Result<Option<ComPtr<HttpExpectationHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Expect)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_from(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_From)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_from(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_From)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn get_host(&self) -> Result<Option<ComPtr<crate::windows::networking::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Host)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn set_host(&self, value: &ComPtr<crate::windows::networking::HostName>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Host)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_if_modified_since(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_IfModifiedSince)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_if_modified_since(&self, value: &ComPtr<foundation::IReference<foundation::DateTime>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_IfModifiedSince)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_if_unmodified_since(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_IfUnmodifiedSince)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_if_unmodified_since(&self, value: &ComPtr<foundation::IReference<foundation::DateTime>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_IfUnmodifiedSince)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_forwards(&self) -> Result<Option<ComPtr<foundation::IReference<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MaxForwards)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_max_forwards(&self, value: &ComPtr<foundation::IReference<u32>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MaxForwards)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_proxy_authorization(&self) -> Result<Option<ComPtr<HttpCredentialsHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ProxyAuthorization)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_proxy_authorization(&self, value: &ComPtr<HttpCredentialsHeaderValue>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ProxyAuthorization)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_referer(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Referer)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_referer(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Referer)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_transfer_encoding(&self) -> Result<Option<ComPtr<HttpTransferCodingHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_TransferEncoding)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_user_agent(&self) -> Result<Option<ComPtr<HttpProductInfoHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_UserAgent)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn append(&self, name: &HStringArg, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Append)(self.deref() as *const _ as *mut _, name.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_append_without_validation(&self, name: &HStringArg, value: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryAppendWithoutValidation)(self.deref() as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpRequestHeaderCollection: IHttpRequestHeaderCollection}
DEFINE_IID!(IID_IHttpResponseHeaderCollection, 2056849769, 64063, 16877, 170, 198, 191, 149, 121, 117, 193, 107);
RT_INTERFACE!{interface IHttpResponseHeaderCollection(IHttpResponseHeaderCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpResponseHeaderCollection] {
    fn get_Age(&self, out: *mut *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn put_Age(&self, value: *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn get_Allow(&self, out: *mut *mut HttpMethodHeaderValueCollection) -> HRESULT,
    fn get_CacheControl(&self, out: *mut *mut HttpCacheDirectiveHeaderValueCollection) -> HRESULT,
    fn get_Connection(&self, out: *mut *mut HttpConnectionOptionHeaderValueCollection) -> HRESULT,
    fn get_Date(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn put_Date(&self, value: *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_Location(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_Location(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_ProxyAuthenticate(&self, out: *mut *mut HttpChallengeHeaderValueCollection) -> HRESULT,
    fn get_RetryAfter(&self, out: *mut *mut HttpDateOrDeltaHeaderValue) -> HRESULT,
    fn put_RetryAfter(&self, value: *mut HttpDateOrDeltaHeaderValue) -> HRESULT,
    fn get_TransferEncoding(&self, out: *mut *mut HttpTransferCodingHeaderValueCollection) -> HRESULT,
    fn get_WwwAuthenticate(&self, out: *mut *mut HttpChallengeHeaderValueCollection) -> HRESULT,
    fn Append(&self, name: HSTRING, value: HSTRING) -> HRESULT,
    fn TryAppendWithoutValidation(&self, name: HSTRING, value: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpResponseHeaderCollection> {
    #[inline] pub fn get_age(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::TimeSpan>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Age)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_age(&self, value: &ComPtr<foundation::IReference<foundation::TimeSpan>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Age)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow(&self) -> Result<Option<ComPtr<HttpMethodHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Allow)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cache_control(&self) -> Result<Option<ComPtr<HttpCacheDirectiveHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CacheControl)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_connection(&self) -> Result<Option<ComPtr<HttpConnectionOptionHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Connection)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_date(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Date)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_date(&self, value: &ComPtr<foundation::IReference<foundation::DateTime>>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Date)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_location(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Location)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_location(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Location)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_proxy_authenticate(&self) -> Result<Option<ComPtr<HttpChallengeHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ProxyAuthenticate)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_retry_after(&self) -> Result<Option<ComPtr<HttpDateOrDeltaHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_RetryAfter)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_retry_after(&self, value: &ComPtr<HttpDateOrDeltaHeaderValue>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_RetryAfter)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_transfer_encoding(&self) -> Result<Option<ComPtr<HttpTransferCodingHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_TransferEncoding)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_www_authenticate(&self) -> Result<Option<ComPtr<HttpChallengeHeaderValueCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_WwwAuthenticate)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn append(&self, name: &HStringArg, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Append)(self.deref() as *const _ as *mut _, name.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_append_without_validation(&self, name: &HStringArg, value: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryAppendWithoutValidation)(self.deref() as *const _ as *mut _, name.get(), value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpResponseHeaderCollection: IHttpResponseHeaderCollection}
DEFINE_IID!(IID_IHttpTransferCodingHeaderValue, 1131361017, 15853, 17085, 179, 138, 84, 150, 162, 81, 28, 230);
RT_INTERFACE!{interface IHttpTransferCodingHeaderValue(IHttpTransferCodingHeaderValueVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransferCodingHeaderValue] {
    fn get_Parameters(&self, out: *mut *mut foundation::collections::IVector<HttpNameValueHeaderValue>) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHttpTransferCodingHeaderValue> {
    #[inline] pub fn get_parameters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HttpNameValueHeaderValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Parameters)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpTransferCodingHeaderValue: IHttpTransferCodingHeaderValue}
impl RtActivatable<IHttpTransferCodingHeaderValueFactory> for HttpTransferCodingHeaderValue {}
impl RtActivatable<IHttpTransferCodingHeaderValueStatics> for HttpTransferCodingHeaderValue {}
impl HttpTransferCodingHeaderValue {
    #[inline] pub fn create(input: &HStringArg) -> Result<ComPtr<HttpTransferCodingHeaderValue>> {
        <Self as RtActivatable<IHttpTransferCodingHeaderValueFactory>>::get_activation_factory().create(input)
    }
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<HttpTransferCodingHeaderValue>>> {
        <Self as RtActivatable<IHttpTransferCodingHeaderValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<HttpTransferCodingHeaderValue>>, bool)> {
        <Self as RtActivatable<IHttpTransferCodingHeaderValueStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(HttpTransferCodingHeaderValue(&[87,105,110,100,111,119,115,46,87,101,98,46,72,116,116,112,46,72,101,97,100,101,114,115,46,72,116,116,112,84,114,97,110,115,102,101,114,67,111,100,105,110,103,72,101,97,100,101,114,86,97,108,117,101,0]) [CLSID_HttpTransferCodingHeaderValue]);
DEFINE_IID!(IID_IHttpTransferCodingHeaderValueCollection, 539790388, 11267, 18872, 150, 101, 115, 226, 124, 178, 252, 121);
RT_INTERFACE!{interface IHttpTransferCodingHeaderValueCollection(IHttpTransferCodingHeaderValueCollectionVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransferCodingHeaderValueCollection] {
    fn ParseAdd(&self, input: HSTRING) -> HRESULT,
    fn TryParseAdd(&self, input: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpTransferCodingHeaderValueCollection> {
    #[inline] pub fn parse_add(&self, input: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).ParseAdd)(self.deref() as *const _ as *mut _, input.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_parse_add(&self, input: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParseAdd)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HttpTransferCodingHeaderValueCollection: IHttpTransferCodingHeaderValueCollection}
DEFINE_IID!(IID_IHttpTransferCodingHeaderValueFactory, 3143819260, 58209, 20232, 142, 79, 201, 231, 35, 222, 112, 59);
RT_INTERFACE!{static interface IHttpTransferCodingHeaderValueFactory(IHttpTransferCodingHeaderValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransferCodingHeaderValueFactory] {
    fn Create(&self, input: HSTRING, out: *mut *mut HttpTransferCodingHeaderValue) -> HRESULT
}}
impl ComPtr<IHttpTransferCodingHeaderValueFactory> {
    #[inline] pub fn create(&self, input: &HStringArg) -> Result<ComPtr<HttpTransferCodingHeaderValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Create)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHttpTransferCodingHeaderValueStatics, 1790478634, 6808, 19762, 169, 6, 116, 112, 169, 135, 92, 229);
RT_INTERFACE!{static interface IHttpTransferCodingHeaderValueStatics(IHttpTransferCodingHeaderValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHttpTransferCodingHeaderValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut HttpTransferCodingHeaderValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, transferCodingHeaderValue: *mut *mut HttpTransferCodingHeaderValue, out: *mut bool) -> HRESULT
}}
impl ComPtr<IHttpTransferCodingHeaderValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<HttpTransferCodingHeaderValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).Parse)(self.deref() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<HttpTransferCodingHeaderValue>>, bool)> { unsafe { 
        let mut transferCodingHeaderValue = null_mut(); let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).TryParse)(self.deref() as *const _ as *mut _, input.get(), &mut transferCodingHeaderValue, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(transferCodingHeaderValue), out)) } else { err(hr) }
    }}
}
} // Windows.Web.Http.Headers
} // Windows.Web.Http
pub mod syndication { // Windows.Web.Syndication
use crate::prelude::*;
RT_STRUCT! { struct RetrievalProgress {
    BytesRetrieved: u32, TotalBytesToRetrieve: u32,
}}
DEFINE_IID!(IID_ISyndicationAttribute, 1911093609, 21102, 16385, 154, 145, 232, 79, 131, 22, 26, 177);
RT_INTERFACE!{interface ISyndicationAttribute(ISyndicationAttributeVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationAttribute] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_Namespace(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Namespace(&self, value: HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<ISyndicationAttribute> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Name)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Namespace)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_namespace(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Namespace)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Value)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationAttribute: ISyndicationAttribute}
impl RtActivatable<ISyndicationAttributeFactory> for SyndicationAttribute {}
impl RtActivatable<IActivationFactory> for SyndicationAttribute {}
impl SyndicationAttribute {
    #[inline] pub fn create_syndication_attribute(attributeName: &HStringArg, attributeNamespace: &HStringArg, attributeValue: &HStringArg) -> Result<ComPtr<SyndicationAttribute>> {
        <Self as RtActivatable<ISyndicationAttributeFactory>>::get_activation_factory().create_syndication_attribute(attributeName, attributeNamespace, attributeValue)
    }
}
DEFINE_CLSID!(SyndicationAttribute(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,65,116,116,114,105,98,117,116,101,0]) [CLSID_SyndicationAttribute]);
DEFINE_IID!(IID_ISyndicationAttributeFactory, 1649350041, 60734, 16911, 190, 134, 100, 4, 20, 136, 110, 75);
RT_INTERFACE!{static interface ISyndicationAttributeFactory(ISyndicationAttributeFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationAttributeFactory] {
    fn CreateSyndicationAttribute(&self, attributeName: HSTRING, attributeNamespace: HSTRING, attributeValue: HSTRING, out: *mut *mut SyndicationAttribute) -> HRESULT
}}
impl ComPtr<ISyndicationAttributeFactory> {
    #[inline] pub fn create_syndication_attribute(&self, attributeName: &HStringArg, attributeNamespace: &HStringArg, attributeValue: &HStringArg) -> Result<ComPtr<SyndicationAttribute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationAttribute)(self.deref() as *const _ as *mut _, attributeName.get(), attributeNamespace.get(), attributeValue.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISyndicationCategory, 2266325615, 3258, 19071, 137, 255, 236, 181, 40, 20, 35, 182);
RT_INTERFACE!{interface ISyndicationCategory(ISyndicationCategoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationCategory] {
    fn get_Label(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Label(&self, value: HSTRING) -> HRESULT,
    fn get_Scheme(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Scheme(&self, value: HSTRING) -> HRESULT,
    fn get_Term(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Term(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<ISyndicationCategory> {
    #[inline] pub fn get_label(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Label)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_label(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Label)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_scheme(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Scheme)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_scheme(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Scheme)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_term(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Term)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_term(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Term)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationCategory: ISyndicationCategory}
impl RtActivatable<ISyndicationCategoryFactory> for SyndicationCategory {}
impl RtActivatable<IActivationFactory> for SyndicationCategory {}
impl SyndicationCategory {
    #[inline] pub fn create_syndication_category(term: &HStringArg) -> Result<ComPtr<SyndicationCategory>> {
        <Self as RtActivatable<ISyndicationCategoryFactory>>::get_activation_factory().create_syndication_category(term)
    }
    #[inline] pub fn create_syndication_category_ex(term: &HStringArg, scheme: &HStringArg, label: &HStringArg) -> Result<ComPtr<SyndicationCategory>> {
        <Self as RtActivatable<ISyndicationCategoryFactory>>::get_activation_factory().create_syndication_category_ex(term, scheme, label)
    }
}
DEFINE_CLSID!(SyndicationCategory(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,67,97,116,101,103,111,114,121,0]) [CLSID_SyndicationCategory]);
DEFINE_IID!(IID_ISyndicationCategoryFactory, 2873262127, 18912, 17701, 138, 178, 171, 69, 192, 37, 40, 255);
RT_INTERFACE!{static interface ISyndicationCategoryFactory(ISyndicationCategoryFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationCategoryFactory] {
    fn CreateSyndicationCategory(&self, term: HSTRING, out: *mut *mut SyndicationCategory) -> HRESULT,
    fn CreateSyndicationCategoryEx(&self, term: HSTRING, scheme: HSTRING, label: HSTRING, out: *mut *mut SyndicationCategory) -> HRESULT
}}
impl ComPtr<ISyndicationCategoryFactory> {
    #[inline] pub fn create_syndication_category(&self, term: &HStringArg) -> Result<ComPtr<SyndicationCategory>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationCategory)(self.deref() as *const _ as *mut _, term.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_syndication_category_ex(&self, term: &HStringArg, scheme: &HStringArg, label: &HStringArg) -> Result<ComPtr<SyndicationCategory>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationCategoryEx)(self.deref() as *const _ as *mut _, term.get(), scheme.get(), label.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISyndicationClient, 2652416439, 29257, 19269, 178, 41, 125, 248, 149, 165, 161, 245);
RT_INTERFACE!{interface ISyndicationClient(ISyndicationClientVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationClient] {
    #[cfg(not(feature="windows-security"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ServerCredential(&self, value: *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ProxyCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ProxyCredential(&self, value: *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    fn get_MaxResponseBufferSize(&self, out: *mut u32) -> HRESULT,
    fn put_MaxResponseBufferSize(&self, value: u32) -> HRESULT,
    fn get_Timeout(&self, out: *mut u32) -> HRESULT,
    fn put_Timeout(&self, value: u32) -> HRESULT,
    fn get_BypassCacheOnRetrieve(&self, out: *mut bool) -> HRESULT,
    fn put_BypassCacheOnRetrieve(&self, value: bool) -> HRESULT,
    fn SetRequestHeader(&self, name: HSTRING, value: HSTRING) -> HRESULT,
    fn RetrieveFeedAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>) -> HRESULT
}}
impl ComPtr<ISyndicationClient> {
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ServerCredential)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_server_credential(&self, value: &ComPtr<super::super::security::credentials::PasswordCredential>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ServerCredential)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_proxy_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ProxyCredential)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_proxy_credential(&self, value: &ComPtr<super::super::security::credentials::PasswordCredential>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ProxyCredential)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_response_buffer_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_MaxResponseBufferSize)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_response_buffer_size(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MaxResponseBufferSize)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_timeout(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Timeout)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_timeout(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Timeout)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bypass_cache_on_retrieve(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_BypassCacheOnRetrieve)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_bypass_cache_on_retrieve(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_BypassCacheOnRetrieve)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_request_header(&self, name: &HStringArg, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).SetRequestHeader)(self.deref() as *const _ as *mut _, name.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn retrieve_feed_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).RetrieveFeedAsync)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationClient: ISyndicationClient}
impl RtActivatable<ISyndicationClientFactory> for SyndicationClient {}
impl RtActivatable<IActivationFactory> for SyndicationClient {}
impl SyndicationClient {
    #[cfg(feature="windows-security")] #[inline] pub fn create_syndication_client(serverCredential: &ComPtr<super::super::security::credentials::PasswordCredential>) -> Result<ComPtr<SyndicationClient>> {
        <Self as RtActivatable<ISyndicationClientFactory>>::get_activation_factory().create_syndication_client(serverCredential)
    }
}
DEFINE_CLSID!(SyndicationClient(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,67,108,105,101,110,116,0]) [CLSID_SyndicationClient]);
DEFINE_IID!(IID_ISyndicationClientFactory, 784642860, 42907, 16660, 178, 154, 5, 223, 251, 175, 185, 164);
RT_INTERFACE!{static interface ISyndicationClientFactory(ISyndicationClientFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationClientFactory] {
    #[cfg(feature="windows-security")] fn CreateSyndicationClient(&self, serverCredential: *mut super::super::security::credentials::PasswordCredential, out: *mut *mut SyndicationClient) -> HRESULT
}}
impl ComPtr<ISyndicationClientFactory> {
    #[cfg(feature="windows-security")] #[inline] pub fn create_syndication_client(&self, serverCredential: &ComPtr<super::super::security::credentials::PasswordCredential>) -> Result<ComPtr<SyndicationClient>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationClient)(self.deref() as *const _ as *mut _, serverCredential.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISyndicationContent, 1178730238, 3669, 16592, 184, 208, 106, 44, 203, 169, 252, 124);
RT_INTERFACE!{interface ISyndicationContent(ISyndicationContentVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationContent] {
    fn get_SourceUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_SourceUri(&self, value: *mut foundation::Uri) -> HRESULT
}}
impl ComPtr<ISyndicationContent> {
    #[inline] pub fn get_source_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_SourceUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_source_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_SourceUri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationContent: ISyndicationContent}
impl RtActivatable<ISyndicationContentFactory> for SyndicationContent {}
impl RtActivatable<IActivationFactory> for SyndicationContent {}
impl SyndicationContent {
    #[inline] pub fn create_syndication_content(text: &HStringArg, type_: SyndicationTextType) -> Result<ComPtr<SyndicationContent>> {
        <Self as RtActivatable<ISyndicationContentFactory>>::get_activation_factory().create_syndication_content(text, type_)
    }
    #[inline] pub fn create_syndication_content_with_source_uri(sourceUri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationContent>> {
        <Self as RtActivatable<ISyndicationContentFactory>>::get_activation_factory().create_syndication_content_with_source_uri(sourceUri)
    }
}
DEFINE_CLSID!(SyndicationContent(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,67,111,110,116,101,110,116,0]) [CLSID_SyndicationContent]);
DEFINE_IID!(IID_ISyndicationContentFactory, 1026538387, 38176, 16755, 147, 136, 126, 45, 243, 36, 168, 160);
RT_INTERFACE!{static interface ISyndicationContentFactory(ISyndicationContentFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationContentFactory] {
    fn CreateSyndicationContent(&self, text: HSTRING, type_: SyndicationTextType, out: *mut *mut SyndicationContent) -> HRESULT,
    fn CreateSyndicationContentWithSourceUri(&self, sourceUri: *mut foundation::Uri, out: *mut *mut SyndicationContent) -> HRESULT
}}
impl ComPtr<ISyndicationContentFactory> {
    #[inline] pub fn create_syndication_content(&self, text: &HStringArg, type_: SyndicationTextType) -> Result<ComPtr<SyndicationContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationContent)(self.deref() as *const _ as *mut _, text.get(), type_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_syndication_content_with_source_uri(&self, sourceUri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationContentWithSourceUri)(self.deref() as *const _ as *mut _, sourceUri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class SyndicationError}
impl RtActivatable<ISyndicationErrorStatics> for SyndicationError {}
impl SyndicationError {
    #[inline] pub fn get_status(hresult: i32) -> Result<SyndicationErrorStatus> {
        <Self as RtActivatable<ISyndicationErrorStatics>>::get_activation_factory().get_status(hresult)
    }
}
DEFINE_CLSID!(SyndicationError(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,69,114,114,111,114,0]) [CLSID_SyndicationError]);
DEFINE_IID!(IID_ISyndicationErrorStatics, 532357985, 17863, 18483, 138, 160, 190, 95, 59, 88, 167, 244);
RT_INTERFACE!{static interface ISyndicationErrorStatics(ISyndicationErrorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationErrorStatics] {
    fn GetStatus(&self, hresult: i32, out: *mut SyndicationErrorStatus) -> HRESULT
}}
impl ComPtr<ISyndicationErrorStatics> {
    #[inline] pub fn get_status(&self, hresult: i32) -> Result<SyndicationErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).GetStatus)(self.deref() as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum SyndicationErrorStatus: i32 {
    Unknown = 0, MissingRequiredElement = 1, MissingRequiredAttribute = 2, InvalidXml = 3, UnexpectedContent = 4, UnsupportedFormat = 5,
}}
DEFINE_IID!(IID_ISyndicationFeed, 2147368146, 23398, 19810, 132, 3, 27, 193, 13, 145, 13, 107);
RT_INTERFACE!{interface ISyndicationFeed(ISyndicationFeedVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationFeed] {
    fn get_Authors(&self, out: *mut *mut foundation::collections::IVector<SyndicationPerson>) -> HRESULT,
    fn get_Categories(&self, out: *mut *mut foundation::collections::IVector<SyndicationCategory>) -> HRESULT,
    fn get_Contributors(&self, out: *mut *mut foundation::collections::IVector<SyndicationPerson>) -> HRESULT,
    fn get_Generator(&self, out: *mut *mut SyndicationGenerator) -> HRESULT,
    fn put_Generator(&self, value: *mut SyndicationGenerator) -> HRESULT,
    fn get_IconUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_IconUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Id(&self, value: HSTRING) -> HRESULT,
    fn get_Items(&self, out: *mut *mut foundation::collections::IVector<SyndicationItem>) -> HRESULT,
    fn get_LastUpdatedTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn put_LastUpdatedTime(&self, value: foundation::DateTime) -> HRESULT,
    fn get_Links(&self, out: *mut *mut foundation::collections::IVector<SyndicationLink>) -> HRESULT,
    fn get_ImageUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_ImageUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_Rights(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Rights(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_Subtitle(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Subtitle(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_Title(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Title(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_FirstUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_LastUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_NextUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_PreviousUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_SourceFormat(&self, out: *mut SyndicationFormat) -> HRESULT,
    fn Load(&self, feed: HSTRING) -> HRESULT,
    #[cfg(feature="windows-data")] fn LoadFromXml(&self, feedDocument: *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl ComPtr<ISyndicationFeed> {
    #[inline] pub fn get_authors(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationPerson>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Authors)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_categories(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationCategory>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Categories)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_contributors(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationPerson>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Contributors)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_generator(&self) -> Result<Option<ComPtr<SyndicationGenerator>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Generator)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_generator(&self, value: &ComPtr<SyndicationGenerator>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Generator)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_icon_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_IconUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_icon_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_IconUri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Id)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Id)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_items(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Items)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_updated_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_LastUpdatedTime)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_last_updated_time(&self, value: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_LastUpdatedTime)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_links(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationLink>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Links)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_image_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ImageUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_image_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ImageUri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_rights(&self) -> Result<Option<ComPtr<ISyndicationText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Rights)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_rights(&self, value: &ComPtr<ISyndicationText>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Rights)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_subtitle(&self) -> Result<Option<ComPtr<ISyndicationText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Subtitle)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_subtitle(&self, value: &ComPtr<ISyndicationText>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Subtitle)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<Option<ComPtr<ISyndicationText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Title)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &ComPtr<ISyndicationText>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Title)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_first_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_FirstUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_LastUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_NextUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_previous_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_PreviousUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_format(&self) -> Result<SyndicationFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_SourceFormat)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn load(&self, feed: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Load)(self.deref() as *const _ as *mut _, feed.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn load_from_xml(&self, feedDocument: &ComPtr<super::super::data::xml::dom::XmlDocument>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).LoadFromXml)(self.deref() as *const _ as *mut _, feedDocument.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationFeed: ISyndicationFeed}
impl RtActivatable<ISyndicationFeedFactory> for SyndicationFeed {}
impl RtActivatable<IActivationFactory> for SyndicationFeed {}
impl SyndicationFeed {
    #[inline] pub fn create_syndication_feed(title: &HStringArg, subtitle: &HStringArg, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationFeed>> {
        <Self as RtActivatable<ISyndicationFeedFactory>>::get_activation_factory().create_syndication_feed(title, subtitle, uri)
    }
}
DEFINE_CLSID!(SyndicationFeed(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,70,101,101,100,0]) [CLSID_SyndicationFeed]);
DEFINE_IID!(IID_ISyndicationFeedFactory, 591864370, 35817, 18615, 137, 52, 98, 5, 19, 29, 147, 87);
RT_INTERFACE!{static interface ISyndicationFeedFactory(ISyndicationFeedFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationFeedFactory] {
    fn CreateSyndicationFeed(&self, title: HSTRING, subtitle: HSTRING, uri: *mut foundation::Uri, out: *mut *mut SyndicationFeed) -> HRESULT
}}
impl ComPtr<ISyndicationFeedFactory> {
    #[inline] pub fn create_syndication_feed(&self, title: &HStringArg, subtitle: &HStringArg, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationFeed>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationFeed)(self.deref() as *const _ as *mut _, title.get(), subtitle.get(), uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SyndicationFormat: i32 {
    Atom10 = 0, Rss20 = 1, Rss10 = 2, Rss092 = 3, Rss091 = 4, Atom03 = 5,
}}
DEFINE_IID!(IID_ISyndicationGenerator, 2540221305, 64299, 20333, 180, 28, 8, 138, 88, 104, 130, 92);
RT_INTERFACE!{interface ISyndicationGenerator(ISyndicationGeneratorVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationGenerator] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Text(&self, value: HSTRING) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_Uri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_Version(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Version(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<ISyndicationGenerator> {
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Text)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Text)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Uri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Version)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_version(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Version)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationGenerator: ISyndicationGenerator}
impl RtActivatable<ISyndicationGeneratorFactory> for SyndicationGenerator {}
impl RtActivatable<IActivationFactory> for SyndicationGenerator {}
impl SyndicationGenerator {
    #[inline] pub fn create_syndication_generator(text: &HStringArg) -> Result<ComPtr<SyndicationGenerator>> {
        <Self as RtActivatable<ISyndicationGeneratorFactory>>::get_activation_factory().create_syndication_generator(text)
    }
}
DEFINE_CLSID!(SyndicationGenerator(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_SyndicationGenerator]);
DEFINE_IID!(IID_ISyndicationGeneratorFactory, 2738914275, 7718, 19900, 186, 157, 26, 184, 75, 239, 249, 123);
RT_INTERFACE!{static interface ISyndicationGeneratorFactory(ISyndicationGeneratorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationGeneratorFactory] {
    fn CreateSyndicationGenerator(&self, text: HSTRING, out: *mut *mut SyndicationGenerator) -> HRESULT
}}
impl ComPtr<ISyndicationGeneratorFactory> {
    #[inline] pub fn create_syndication_generator(&self, text: &HStringArg) -> Result<ComPtr<SyndicationGenerator>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationGenerator)(self.deref() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISyndicationItem, 1418573955, 50052, 17857, 138, 232, 163, 120, 196, 236, 72, 108);
RT_INTERFACE!{interface ISyndicationItem(ISyndicationItemVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationItem] {
    fn get_Authors(&self, out: *mut *mut foundation::collections::IVector<SyndicationPerson>) -> HRESULT,
    fn get_Categories(&self, out: *mut *mut foundation::collections::IVector<SyndicationCategory>) -> HRESULT,
    fn get_Contributors(&self, out: *mut *mut foundation::collections::IVector<SyndicationPerson>) -> HRESULT,
    fn get_Content(&self, out: *mut *mut SyndicationContent) -> HRESULT,
    fn put_Content(&self, value: *mut SyndicationContent) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Id(&self, value: HSTRING) -> HRESULT,
    fn get_LastUpdatedTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn put_LastUpdatedTime(&self, value: foundation::DateTime) -> HRESULT,
    fn get_Links(&self, out: *mut *mut foundation::collections::IVector<SyndicationLink>) -> HRESULT,
    fn get_PublishedDate(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn put_PublishedDate(&self, value: foundation::DateTime) -> HRESULT,
    fn get_Rights(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Rights(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_Source(&self, out: *mut *mut SyndicationFeed) -> HRESULT,
    fn put_Source(&self, value: *mut SyndicationFeed) -> HRESULT,
    fn get_Summary(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Summary(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_Title(&self, out: *mut *mut ISyndicationText) -> HRESULT,
    fn put_Title(&self, value: *mut ISyndicationText) -> HRESULT,
    fn get_CommentsUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_CommentsUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_EditUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_EditMediaUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_ETag(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ItemUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn Load(&self, item: HSTRING) -> HRESULT,
    #[cfg(feature="windows-data")] fn LoadFromXml(&self, itemDocument: *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl ComPtr<ISyndicationItem> {
    #[inline] pub fn get_authors(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationPerson>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Authors)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_categories(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationCategory>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Categories)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_contributors(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationPerson>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Contributors)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_content(&self) -> Result<Option<ComPtr<SyndicationContent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Content)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content(&self, value: &ComPtr<SyndicationContent>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Content)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Id)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Id)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_last_updated_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_LastUpdatedTime)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_last_updated_time(&self, value: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_LastUpdatedTime)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_links(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationLink>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Links)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_published_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_PublishedDate)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_published_date(&self, value: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_PublishedDate)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_rights(&self) -> Result<Option<ComPtr<ISyndicationText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Rights)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_rights(&self, value: &ComPtr<ISyndicationText>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Rights)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_source(&self) -> Result<Option<ComPtr<SyndicationFeed>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Source)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_source(&self, value: &ComPtr<SyndicationFeed>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Source)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_summary(&self) -> Result<Option<ComPtr<ISyndicationText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Summary)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_summary(&self, value: &ComPtr<ISyndicationText>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Summary)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<Option<ComPtr<ISyndicationText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Title)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &ComPtr<ISyndicationText>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Title)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_comments_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_CommentsUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_comments_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_CommentsUri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_edit_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_EditUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_edit_media_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_EditMediaUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_etag(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ETag)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_item_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ItemUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn load(&self, item: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Load)(self.deref() as *const _ as *mut _, item.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn load_from_xml(&self, itemDocument: &ComPtr<super::super::data::xml::dom::XmlDocument>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).LoadFromXml)(self.deref() as *const _ as *mut _, itemDocument.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationItem: ISyndicationItem}
impl RtActivatable<ISyndicationItemFactory> for SyndicationItem {}
impl RtActivatable<IActivationFactory> for SyndicationItem {}
impl SyndicationItem {
    #[inline] pub fn create_syndication_item(title: &HStringArg, content: &ComPtr<SyndicationContent>, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationItem>> {
        <Self as RtActivatable<ISyndicationItemFactory>>::get_activation_factory().create_syndication_item(title, content, uri)
    }
}
DEFINE_CLSID!(SyndicationItem(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,73,116,101,109,0]) [CLSID_SyndicationItem]);
DEFINE_IID!(IID_ISyndicationItemFactory, 622674767, 32184, 18554, 133, 228, 16, 209, 145, 230, 110, 187);
RT_INTERFACE!{static interface ISyndicationItemFactory(ISyndicationItemFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationItemFactory] {
    fn CreateSyndicationItem(&self, title: HSTRING, content: *mut SyndicationContent, uri: *mut foundation::Uri, out: *mut *mut SyndicationItem) -> HRESULT
}}
impl ComPtr<ISyndicationItemFactory> {
    #[inline] pub fn create_syndication_item(&self, title: &HStringArg, content: &ComPtr<SyndicationContent>, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationItem)(self.deref() as *const _ as *mut _, title.get(), content.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISyndicationLink, 659897021, 41230, 16821, 134, 189, 151, 89, 8, 110, 176, 197);
RT_INTERFACE!{interface ISyndicationLink(ISyndicationLinkVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationLink] {
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn put_Length(&self, value: u32) -> HRESULT,
    fn get_MediaType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_MediaType(&self, value: HSTRING) -> HRESULT,
    fn get_Relationship(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Relationship(&self, value: HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_Uri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_ResourceLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ResourceLanguage(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<ISyndicationLink> {
    #[inline] pub fn get_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Length)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_length(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Length)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MediaType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_media_type(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_MediaType)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_relationship(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Relationship)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_relationship(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Relationship)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Title)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Title)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Uri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_resource_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ResourceLanguage)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_resource_language(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_ResourceLanguage)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationLink: ISyndicationLink}
impl RtActivatable<ISyndicationLinkFactory> for SyndicationLink {}
impl RtActivatable<IActivationFactory> for SyndicationLink {}
impl SyndicationLink {
    #[inline] pub fn create_syndication_link(uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationLink>> {
        <Self as RtActivatable<ISyndicationLinkFactory>>::get_activation_factory().create_syndication_link(uri)
    }
    #[inline] pub fn create_syndication_link_ex(uri: &ComPtr<foundation::Uri>, relationship: &HStringArg, title: &HStringArg, mediaType: &HStringArg, length: u32) -> Result<ComPtr<SyndicationLink>> {
        <Self as RtActivatable<ISyndicationLinkFactory>>::get_activation_factory().create_syndication_link_ex(uri, relationship, title, mediaType, length)
    }
}
DEFINE_CLSID!(SyndicationLink(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,76,105,110,107,0]) [CLSID_SyndicationLink]);
DEFINE_IID!(IID_ISyndicationLinkFactory, 1591239636, 21813, 18604, 152, 212, 193, 144, 153, 80, 128, 179);
RT_INTERFACE!{static interface ISyndicationLinkFactory(ISyndicationLinkFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationLinkFactory] {
    fn CreateSyndicationLink(&self, uri: *mut foundation::Uri, out: *mut *mut SyndicationLink) -> HRESULT,
    fn CreateSyndicationLinkEx(&self, uri: *mut foundation::Uri, relationship: HSTRING, title: HSTRING, mediaType: HSTRING, length: u32, out: *mut *mut SyndicationLink) -> HRESULT
}}
impl ComPtr<ISyndicationLinkFactory> {
    #[inline] pub fn create_syndication_link(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationLink>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationLink)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_syndication_link_ex(&self, uri: &ComPtr<foundation::Uri>, relationship: &HStringArg, title: &HStringArg, mediaType: &HStringArg, length: u32) -> Result<ComPtr<SyndicationLink>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationLinkEx)(self.deref() as *const _ as *mut _, uri.deref() as *const _ as *mut _, relationship.get(), title.get(), mediaType.get(), length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISyndicationNode, 1966927736, 20984, 17856, 169, 245, 241, 113, 157, 236, 63, 178);
RT_INTERFACE!{interface ISyndicationNode(ISyndicationNodeVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationNode] {
    fn get_NodeName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NodeName(&self, value: HSTRING) -> HRESULT,
    fn get_NodeNamespace(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NodeNamespace(&self, value: HSTRING) -> HRESULT,
    fn get_NodeValue(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NodeValue(&self, value: HSTRING) -> HRESULT,
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Language(&self, value: HSTRING) -> HRESULT,
    fn get_BaseUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_BaseUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_AttributeExtensions(&self, out: *mut *mut foundation::collections::IVector<SyndicationAttribute>) -> HRESULT,
    fn get_ElementExtensions(&self, out: *mut *mut foundation::collections::IVector<ISyndicationNode>) -> HRESULT,
    #[cfg(feature="windows-data")] fn GetXmlDocument(&self, format: SyndicationFormat, out: *mut *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl ComPtr<ISyndicationNode> {
    #[inline] pub fn get_node_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_NodeName)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_node_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_NodeName)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_node_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_NodeNamespace)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_node_namespace(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_NodeNamespace)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_node_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_NodeValue)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_node_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_NodeValue)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Language)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_language(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Language)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_base_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_BaseUri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_base_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_BaseUri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute_extensions(&self) -> Result<Option<ComPtr<foundation::collections::IVector<SyndicationAttribute>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_AttributeExtensions)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_element_extensions(&self) -> Result<Option<ComPtr<foundation::collections::IVector<ISyndicationNode>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_ElementExtensions)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_document(&self, format: SyndicationFormat) -> Result<Option<ComPtr<super::super::data::xml::dom::XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetXmlDocument)(self.deref() as *const _ as *mut _, format, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationNode: ISyndicationNode}
impl RtActivatable<ISyndicationNodeFactory> for SyndicationNode {}
impl RtActivatable<IActivationFactory> for SyndicationNode {}
impl SyndicationNode {
    #[inline] pub fn create_syndication_node(nodeName: &HStringArg, nodeNamespace: &HStringArg, nodeValue: &HStringArg) -> Result<ComPtr<SyndicationNode>> {
        <Self as RtActivatable<ISyndicationNodeFactory>>::get_activation_factory().create_syndication_node(nodeName, nodeNamespace, nodeValue)
    }
}
DEFINE_CLSID!(SyndicationNode(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,78,111,100,101,0]) [CLSID_SyndicationNode]);
DEFINE_IID!(IID_ISyndicationNodeFactory, 311435656, 19147, 18856, 183, 119, 165, 235, 146, 225, 138, 121);
RT_INTERFACE!{static interface ISyndicationNodeFactory(ISyndicationNodeFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationNodeFactory] {
    fn CreateSyndicationNode(&self, nodeName: HSTRING, nodeNamespace: HSTRING, nodeValue: HSTRING, out: *mut *mut SyndicationNode) -> HRESULT
}}
impl ComPtr<ISyndicationNodeFactory> {
    #[inline] pub fn create_syndication_node(&self, nodeName: &HStringArg, nodeNamespace: &HStringArg, nodeValue: &HStringArg) -> Result<ComPtr<SyndicationNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationNode)(self.deref() as *const _ as *mut _, nodeName.get(), nodeNamespace.get(), nodeValue.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISyndicationPerson, 4196328922, 42950, 17687, 160, 150, 1, 67, 250, 242, 147, 39);
RT_INTERFACE!{interface ISyndicationPerson(ISyndicationPersonVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationPerson] {
    fn get_Email(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Email(&self, value: HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_Uri(&self, value: *mut foundation::Uri) -> HRESULT
}}
impl ComPtr<ISyndicationPerson> {
    #[inline] pub fn get_email(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Email)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_email(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Email)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Name)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Name)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_uri(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Uri)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationPerson: ISyndicationPerson}
impl RtActivatable<ISyndicationPersonFactory> for SyndicationPerson {}
impl RtActivatable<IActivationFactory> for SyndicationPerson {}
impl SyndicationPerson {
    #[inline] pub fn create_syndication_person(name: &HStringArg) -> Result<ComPtr<SyndicationPerson>> {
        <Self as RtActivatable<ISyndicationPersonFactory>>::get_activation_factory().create_syndication_person(name)
    }
    #[inline] pub fn create_syndication_person_ex(name: &HStringArg, email: &HStringArg, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationPerson>> {
        <Self as RtActivatable<ISyndicationPersonFactory>>::get_activation_factory().create_syndication_person_ex(name, email, uri)
    }
}
DEFINE_CLSID!(SyndicationPerson(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,80,101,114,115,111,110,0]) [CLSID_SyndicationPerson]);
DEFINE_IID!(IID_ISyndicationPersonFactory, 3707013229, 8861, 19288, 164, 155, 243, 210, 240, 245, 201, 159);
RT_INTERFACE!{static interface ISyndicationPersonFactory(ISyndicationPersonFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationPersonFactory] {
    fn CreateSyndicationPerson(&self, name: HSTRING, out: *mut *mut SyndicationPerson) -> HRESULT,
    fn CreateSyndicationPersonEx(&self, name: HSTRING, email: HSTRING, uri: *mut foundation::Uri, out: *mut *mut SyndicationPerson) -> HRESULT
}}
impl ComPtr<ISyndicationPersonFactory> {
    #[inline] pub fn create_syndication_person(&self, name: &HStringArg) -> Result<ComPtr<SyndicationPerson>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationPerson)(self.deref() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_syndication_person_ex(&self, name: &HStringArg, email: &HStringArg, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<SyndicationPerson>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationPersonEx)(self.deref() as *const _ as *mut _, name.get(), email.get(), uri.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISyndicationText, 3117178496, 12602, 16529, 162, 166, 36, 62, 14, 233, 35, 249);
RT_INTERFACE!{interface ISyndicationText(ISyndicationTextVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationText] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Text(&self, value: HSTRING) -> HRESULT,
    fn get_Type(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Type(&self, value: HSTRING) -> HRESULT,
    #[cfg(feature="windows-data")] fn get_Xml(&self, out: *mut *mut super::super::data::xml::dom::XmlDocument) -> HRESULT,
    #[cfg(feature="windows-data")] fn put_Xml(&self, value: *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl ComPtr<ISyndicationText> {
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Text)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Text)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Type)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_type(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Type)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml(&self) -> Result<Option<ComPtr<super::super::data::xml::dom::XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Xml)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn set_xml(&self, value: &ComPtr<super::super::data::xml::dom::XmlDocument>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Xml)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SyndicationText: ISyndicationText}
impl RtActivatable<ISyndicationTextFactory> for SyndicationText {}
impl RtActivatable<IActivationFactory> for SyndicationText {}
impl SyndicationText {
    #[inline] pub fn create_syndication_text(text: &HStringArg) -> Result<ComPtr<SyndicationText>> {
        <Self as RtActivatable<ISyndicationTextFactory>>::get_activation_factory().create_syndication_text(text)
    }
    #[inline] pub fn create_syndication_text_ex(text: &HStringArg, type_: SyndicationTextType) -> Result<ComPtr<SyndicationText>> {
        <Self as RtActivatable<ISyndicationTextFactory>>::get_activation_factory().create_syndication_text_ex(text, type_)
    }
}
DEFINE_CLSID!(SyndicationText(&[87,105,110,100,111,119,115,46,87,101,98,46,83,121,110,100,105,99,97,116,105,111,110,46,83,121,110,100,105,99,97,116,105,111,110,84,101,120,116,0]) [CLSID_SyndicationText]);
DEFINE_IID!(IID_ISyndicationTextFactory, 4000531191, 4550, 19237, 171, 98, 229, 150, 189, 22, 41, 70);
RT_INTERFACE!{static interface ISyndicationTextFactory(ISyndicationTextFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISyndicationTextFactory] {
    fn CreateSyndicationText(&self, text: HSTRING, out: *mut *mut SyndicationText) -> HRESULT,
    fn CreateSyndicationTextEx(&self, text: HSTRING, type_: SyndicationTextType, out: *mut *mut SyndicationText) -> HRESULT
}}
impl ComPtr<ISyndicationTextFactory> {
    #[inline] pub fn create_syndication_text(&self, text: &HStringArg) -> Result<ComPtr<SyndicationText>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationText)(self.deref() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_syndication_text_ex(&self, text: &HStringArg, type_: SyndicationTextType) -> Result<ComPtr<SyndicationText>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateSyndicationTextEx)(self.deref() as *const _ as *mut _, text.get(), type_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SyndicationTextType: i32 {
    Text = 0, Html = 1, Xhtml = 2,
}}
RT_STRUCT! { struct TransferProgress {
    BytesSent: u32, TotalBytesToSend: u32, BytesRetrieved: u32, TotalBytesToRetrieve: u32,
}}
} // Windows.Web.Syndication
pub mod ui { // Windows.Web.UI
use crate::prelude::*;
DEFINE_IID!(IID_IWebViewControl, 1066537750, 48240, 19418, 145, 54, 201, 67, 112, 137, 159, 171);
RT_INTERFACE!{interface IWebViewControl(IWebViewControlVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControl] {
    fn get_Source(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_Source(&self, source: *mut foundation::Uri) -> HRESULT,
    fn get_DocumentTitle(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CanGoBack(&self, out: *mut bool) -> HRESULT,
    fn get_CanGoForward(&self, out: *mut bool) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-ui")] fn put_DefaultBackgroundColor(&self, value: super::super::ui::Color) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_DefaultBackgroundColor(&self, out: *mut super::super::ui::Color) -> HRESULT,
    fn get_ContainsFullScreenElement(&self, out: *mut bool) -> HRESULT,
    fn get_Settings(&self, out: *mut *mut WebViewControlSettings) -> HRESULT,
    fn get_DeferredPermissionRequests(&self, out: *mut *mut foundation::collections::IVectorView<WebViewControlDeferredPermissionRequest>) -> HRESULT,
    fn GoForward(&self) -> HRESULT,
    fn GoBack(&self) -> HRESULT,
    fn Refresh(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn Navigate(&self, source: *mut foundation::Uri) -> HRESULT,
    fn NavigateToString(&self, text: HSTRING) -> HRESULT,
    fn NavigateToLocalStreamUri(&self, source: *mut foundation::Uri, streamResolver: *mut super::IUriToStreamResolver) -> HRESULT,
    fn NavigateWithHttpRequestMessage(&self, requestMessage: *mut super::http::HttpRequestMessage) -> HRESULT,
    fn InvokeScriptAsync(&self, scriptName: HSTRING, arguments: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy19(&self) -> (),
    #[cfg(feature="windows-storage")] fn CapturePreviewToStreamAsync(&self, stream: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy20(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn CaptureSelectedContentToDataPackageAsync(&self, out: *mut *mut foundation::IAsyncOperation<super::super::applicationmodel::datatransfer::DataPackage>) -> HRESULT,
    fn BuildLocalStreamUri(&self, contentIdentifier: HSTRING, relativePath: HSTRING, out: *mut *mut foundation::Uri) -> HRESULT,
    fn GetDeferredPermissionRequestById(&self, id: u32, result: *mut *mut WebViewControlDeferredPermissionRequest) -> HRESULT,
    fn add_NavigationStarting(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_NavigationStarting(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ContentLoading(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ContentLoading(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_DOMContentLoaded(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DOMContentLoaded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_NavigationCompleted(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_NavigationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_FrameNavigationStarting(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FrameNavigationStarting(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_FrameContentLoading(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FrameContentLoading(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_FrameDOMContentLoaded(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FrameDOMContentLoaded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_FrameNavigationCompleted(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FrameNavigationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ScriptNotify(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ScriptNotify(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_LongRunningScriptDetected(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LongRunningScriptDetected(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_UnsafeContentWarningDisplaying(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UnsafeContentWarningDisplaying(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_UnviewableContentIdentified(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UnviewableContentIdentified(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_PermissionRequested(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PermissionRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_UnsupportedUriSchemeIdentified(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UnsupportedUriSchemeIdentified(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_NewWindowRequested(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_NewWindowRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ContainsFullScreenElementChanged(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ContainsFullScreenElementChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_WebResourceRequested(&self, handler: *mut foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_WebResourceRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<IWebViewControl> {
    #[inline] pub fn get_source(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Source)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_source(&self, source: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Source)(self.deref() as *const _ as *mut _, source.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_document_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_DocumentTitle)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_can_go_back(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_CanGoBack)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_can_go_forward(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_CanGoForward)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_default_background_color(&self, value: super::super::ui::Color) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_DefaultBackgroundColor)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_default_background_color(&self) -> Result<super::super::ui::Color> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_DefaultBackgroundColor)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_contains_full_screen_element(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ContainsFullScreenElement)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_settings(&self) -> Result<Option<ComPtr<WebViewControlSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Settings)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferred_permission_requests(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<WebViewControlDeferredPermissionRequest>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_DeferredPermissionRequests)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn go_forward(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).GoForward)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn go_back(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).GoBack)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn refresh(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Refresh)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Stop)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn navigate(&self, source: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Navigate)(self.deref() as *const _ as *mut _, source.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn navigate_to_string(&self, text: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).NavigateToString)(self.deref() as *const _ as *mut _, text.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn navigate_to_local_stream_uri(&self, source: &ComPtr<foundation::Uri>, streamResolver: &ComPtr<super::IUriToStreamResolver>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).NavigateToLocalStreamUri)(self.deref() as *const _ as *mut _, source.deref() as *const _ as *mut _, streamResolver.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn navigate_with_http_request_message(&self, requestMessage: &ComPtr<super::http::HttpRequestMessage>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).NavigateWithHttpRequestMessage)(self.deref() as *const _ as *mut _, requestMessage.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn invoke_script_async(&self, scriptName: &HStringArg, arguments: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).InvokeScriptAsync)(self.deref() as *const _ as *mut _, scriptName.get(), arguments.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn capture_preview_to_stream_async(&self, stream: &ComPtr<super::super::storage::streams::IRandomAccessStream>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CapturePreviewToStreamAsync)(self.deref() as *const _ as *mut _, stream.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn capture_selected_content_to_data_package_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<super::super::applicationmodel::datatransfer::DataPackage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CaptureSelectedContentToDataPackageAsync)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn build_local_stream_uri(&self, contentIdentifier: &HStringArg, relativePath: &HStringArg) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).BuildLocalStreamUri)(self.deref() as *const _ as *mut _, contentIdentifier.get(), relativePath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferred_permission_request_by_id(&self, id: u32) -> Result<Option<ComPtr<WebViewControlDeferredPermissionRequest>>> { unsafe { 
        let mut result = null_mut();
        let hr = ((*self.deref().lpVtbl).GetDeferredPermissionRequestById)(self.deref() as *const _ as *mut _, id, &mut result);
        if hr == S_OK { Ok(ComPtr::wrap_optional(result)) } else { err(hr) }
    }}
    #[inline] pub fn add_navigation_starting(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_NavigationStarting)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_navigation_starting(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_NavigationStarting)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_content_loading(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_ContentLoading)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_content_loading(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_ContentLoading)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_dom_content_loaded(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_DOMContentLoaded)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_dom_content_loaded(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_DOMContentLoaded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_navigation_completed(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_NavigationCompleted)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_navigation_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_NavigationCompleted)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_frame_navigation_starting(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_FrameNavigationStarting)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_frame_navigation_starting(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_FrameNavigationStarting)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_frame_content_loading(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_FrameContentLoading)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_frame_content_loading(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_FrameContentLoading)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_frame_dom_content_loaded(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_FrameDOMContentLoaded)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_frame_dom_content_loaded(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_FrameDOMContentLoaded)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_frame_navigation_completed(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_FrameNavigationCompleted)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_frame_navigation_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_FrameNavigationCompleted)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_script_notify(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_ScriptNotify)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_script_notify(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_ScriptNotify)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_long_running_script_detected(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_LongRunningScriptDetected)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_long_running_script_detected(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_LongRunningScriptDetected)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_unsafe_content_warning_displaying(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_UnsafeContentWarningDisplaying)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_unsafe_content_warning_displaying(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_UnsafeContentWarningDisplaying)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_unviewable_content_identified(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_UnviewableContentIdentified)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_unviewable_content_identified(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_UnviewableContentIdentified)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_permission_requested(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_PermissionRequested)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_permission_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_PermissionRequested)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_unsupported_uri_scheme_identified(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_UnsupportedUriSchemeIdentified)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_unsupported_uri_scheme_identified(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_UnsupportedUriSchemeIdentified)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_new_window_requested(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_NewWindowRequested)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_new_window_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_NewWindowRequested)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_contains_full_screen_element_changed(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_ContainsFullScreenElementChanged)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_contains_full_screen_element_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_ContainsFullScreenElementChanged)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_web_resource_requested(&self, handler: &ComPtr<foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_WebResourceRequested)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_web_resource_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_WebResourceRequested)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebViewControl2, 1295779577, 51423, 16844, 139, 213, 42, 148, 123, 32, 69, 3);
RT_INTERFACE!{interface IWebViewControl2(IWebViewControl2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControl2] {
    fn AddInitializeScript(&self, script: HSTRING) -> HRESULT
}}
impl ComPtr<IWebViewControl2> {
    #[inline] pub fn add_initialize_script(&self, script: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).AddInitializeScript)(self.deref() as *const _ as *mut _, script.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebViewControlContentLoadingEventArgs, 2587872434, 47547, 16459, 162, 43, 102, 220, 205, 18, 80, 198);
RT_INTERFACE!{interface IWebViewControlContentLoadingEventArgs(IWebViewControlContentLoadingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlContentLoadingEventArgs] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT
}}
impl ComPtr<IWebViewControlContentLoadingEventArgs> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlContentLoadingEventArgs: IWebViewControlContentLoadingEventArgs}
DEFINE_IID!(IID_IWebViewControlDeferredPermissionRequest, 753093088, 55129, 17500, 153, 38, 137, 149, 41, 143, 21, 43);
RT_INTERFACE!{interface IWebViewControlDeferredPermissionRequest(IWebViewControlDeferredPermissionRequestVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlDeferredPermissionRequest] {
    fn get_Id(&self, out: *mut u32) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_PermissionType(&self, out: *mut WebViewControlPermissionType) -> HRESULT,
    fn Allow(&self) -> HRESULT,
    fn Deny(&self) -> HRESULT
}}
impl ComPtr<IWebViewControlDeferredPermissionRequest> {
    #[inline] pub fn get_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Id)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_permission_type(&self) -> Result<WebViewControlPermissionType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_PermissionType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn allow(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Allow)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn deny(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Deny)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlDeferredPermissionRequest: IWebViewControlDeferredPermissionRequest}
DEFINE_IID!(IID_IWebViewControlDOMContentLoadedEventArgs, 3196829704, 38209, 17733, 159, 242, 45, 245, 133, 178, 159, 125);
RT_INTERFACE!{interface IWebViewControlDOMContentLoadedEventArgs(IWebViewControlDOMContentLoadedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlDOMContentLoadedEventArgs] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT
}}
impl ComPtr<IWebViewControlDOMContentLoadedEventArgs> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlDOMContentLoadedEventArgs: IWebViewControlDOMContentLoadedEventArgs}
DEFINE_IID!(IID_IWebViewControlLongRunningScriptDetectedEventArgs, 711875514, 39092, 17852, 187, 235, 15, 105, 206, 73, 197, 153);
RT_INTERFACE!{interface IWebViewControlLongRunningScriptDetectedEventArgs(IWebViewControlLongRunningScriptDetectedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlLongRunningScriptDetectedEventArgs] {
    fn get_ExecutionTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_StopPageScriptExecution(&self, out: *mut bool) -> HRESULT,
    fn put_StopPageScriptExecution(&self, value: bool) -> HRESULT
}}
impl ComPtr<IWebViewControlLongRunningScriptDetectedEventArgs> {
    #[inline] pub fn get_execution_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ExecutionTime)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_stop_page_script_execution(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_StopPageScriptExecution)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_stop_page_script_execution(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_StopPageScriptExecution)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlLongRunningScriptDetectedEventArgs: IWebViewControlLongRunningScriptDetectedEventArgs}
DEFINE_IID!(IID_IWebViewControlNavigationCompletedEventArgs, 541104408, 18965, 19526, 165, 93, 247, 158, 219, 11, 222, 139);
RT_INTERFACE!{interface IWebViewControlNavigationCompletedEventArgs(IWebViewControlNavigationCompletedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlNavigationCompletedEventArgs] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_IsSuccess(&self, out: *mut bool) -> HRESULT,
    fn get_WebErrorStatus(&self, out: *mut super::WebErrorStatus) -> HRESULT
}}
impl ComPtr<IWebViewControlNavigationCompletedEventArgs> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_success(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsSuccess)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_web_error_status(&self) -> Result<super::WebErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_WebErrorStatus)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlNavigationCompletedEventArgs: IWebViewControlNavigationCompletedEventArgs}
DEFINE_IID!(IID_IWebViewControlNavigationStartingEventArgs, 210786245, 2568, 16839, 134, 59, 113, 227, 169, 84, 145, 55);
RT_INTERFACE!{interface IWebViewControlNavigationStartingEventArgs(IWebViewControlNavigationStartingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlNavigationStartingEventArgs] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Cancel(&self, out: *mut bool) -> HRESULT,
    fn put_Cancel(&self, value: bool) -> HRESULT
}}
impl ComPtr<IWebViewControlNavigationStartingEventArgs> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cancel(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Cancel)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_cancel(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Cancel)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlNavigationStartingEventArgs: IWebViewControlNavigationStartingEventArgs}
DEFINE_IID!(IID_IWebViewControlNewWindowRequestedEventArgs, 1039420347, 41252, 18133, 160, 131, 208, 44, 172, 223, 245, 173);
RT_INTERFACE!{interface IWebViewControlNewWindowRequestedEventArgs(IWebViewControlNewWindowRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlNewWindowRequestedEventArgs] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Referrer(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Handled(&self, out: *mut bool) -> HRESULT,
    fn put_Handled(&self, value: bool) -> HRESULT
}}
impl ComPtr<IWebViewControlNewWindowRequestedEventArgs> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_referrer(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Referrer)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_handled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Handled)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_handled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Handled)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlNewWindowRequestedEventArgs: IWebViewControlNewWindowRequestedEventArgs}
DEFINE_IID!(IID_IWebViewControlNewWindowRequestedEventArgs2, 3040631974, 10926, 19452, 146, 185, 195, 14, 146, 180, 128, 152);
RT_INTERFACE!{interface IWebViewControlNewWindowRequestedEventArgs2(IWebViewControlNewWindowRequestedEventArgs2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlNewWindowRequestedEventArgs2] {
    fn get_NewWindow(&self, out: *mut *mut IWebViewControl) -> HRESULT,
    fn put_NewWindow(&self, value: *mut IWebViewControl) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl ComPtr<IWebViewControlNewWindowRequestedEventArgs2> {
    #[inline] pub fn get_new_window(&self) -> Result<Option<ComPtr<IWebViewControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_NewWindow)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_new_window(&self, value: &ComPtr<IWebViewControl>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_NewWindow)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetDeferral)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebViewControlPermissionRequest, 3854336876, 61999, 16610, 149, 178, 119, 41, 248, 64, 235, 127);
RT_INTERFACE!{interface IWebViewControlPermissionRequest(IWebViewControlPermissionRequestVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlPermissionRequest] {
    fn get_Id(&self, out: *mut u32) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_PermissionType(&self, out: *mut WebViewControlPermissionType) -> HRESULT,
    fn get_State(&self, out: *mut WebViewControlPermissionState) -> HRESULT,
    fn Defer(&self) -> HRESULT,
    fn Allow(&self) -> HRESULT,
    fn Deny(&self) -> HRESULT
}}
impl ComPtr<IWebViewControlPermissionRequest> {
    #[inline] pub fn get_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Id)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_permission_type(&self) -> Result<WebViewControlPermissionType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_PermissionType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<WebViewControlPermissionState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_State)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn defer(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Defer)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn allow(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Allow)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn deny(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Deny)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlPermissionRequest: IWebViewControlPermissionRequest}
DEFINE_IID!(IID_IWebViewControlPermissionRequestedEventArgs, 656428369, 9352, 19653, 150, 142, 10, 119, 30, 89, 193, 71);
RT_INTERFACE!{interface IWebViewControlPermissionRequestedEventArgs(IWebViewControlPermissionRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlPermissionRequestedEventArgs] {
    fn get_PermissionRequest(&self, out: *mut *mut WebViewControlPermissionRequest) -> HRESULT
}}
impl ComPtr<IWebViewControlPermissionRequestedEventArgs> {
    #[inline] pub fn get_permission_request(&self) -> Result<Option<ComPtr<WebViewControlPermissionRequest>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_PermissionRequest)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlPermissionRequestedEventArgs: IWebViewControlPermissionRequestedEventArgs}
RT_ENUM! { enum WebViewControlPermissionState: i32 {
    Unknown = 0, Defer = 1, Allow = 2, Deny = 3,
}}
RT_ENUM! { enum WebViewControlPermissionType: i32 {
    Geolocation = 0, UnlimitedIndexedDBQuota = 1, Media = 2, PointerLock = 3, WebNotifications = 4, Screen = 5, ImmersiveView = 6,
}}
DEFINE_IID!(IID_IWebViewControlScriptNotifyEventArgs, 1226696059, 28489, 16827, 181, 145, 81, 184, 91, 129, 112, 55);
RT_INTERFACE!{interface IWebViewControlScriptNotifyEventArgs(IWebViewControlScriptNotifyEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlScriptNotifyEventArgs] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IWebViewControlScriptNotifyEventArgs> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Value)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlScriptNotifyEventArgs: IWebViewControlScriptNotifyEventArgs}
DEFINE_IID!(IID_IWebViewControlSettings, 3382083519, 24216, 19709, 140, 206, 39, 176, 145, 30, 61, 232);
RT_INTERFACE!{interface IWebViewControlSettings(IWebViewControlSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlSettings] {
    fn put_IsJavaScriptEnabled(&self, value: bool) -> HRESULT,
    fn get_IsJavaScriptEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsIndexedDBEnabled(&self, value: bool) -> HRESULT,
    fn get_IsIndexedDBEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsScriptNotifyAllowed(&self, value: bool) -> HRESULT,
    fn get_IsScriptNotifyAllowed(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IWebViewControlSettings> {
    #[inline] pub fn set_is_javascript_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_IsJavaScriptEnabled)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_javascript_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsJavaScriptEnabled)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_indexeddb_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_IsIndexedDBEnabled)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_indexeddb_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsIndexedDBEnabled)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_script_notify_allowed(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_IsScriptNotifyAllowed)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_script_notify_allowed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsScriptNotifyAllowed)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlSettings: IWebViewControlSettings}
DEFINE_IID!(IID_IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs, 3820493124, 58620, 17372, 148, 202, 249, 128, 243, 11, 197, 29);
RT_INTERFACE!{interface IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs(IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Handled(&self, out: *mut bool) -> HRESULT,
    fn put_Handled(&self, value: bool) -> HRESULT
}}
impl ComPtr<IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_handled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Handled)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_handled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Handled)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlUnsupportedUriSchemeIdentifiedEventArgs: IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs}
DEFINE_IID!(IID_IWebViewControlUnviewableContentIdentifiedEventArgs, 1251377371, 35058, 20000, 182, 147, 180, 226, 223, 74, 165, 129);
RT_INTERFACE!{interface IWebViewControlUnviewableContentIdentifiedEventArgs(IWebViewControlUnviewableContentIdentifiedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlUnviewableContentIdentifiedEventArgs] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Referrer(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_MediaType(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IWebViewControlUnviewableContentIdentifiedEventArgs> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Uri)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_referrer(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Referrer)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_media_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_MediaType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlUnviewableContentIdentifiedEventArgs: IWebViewControlUnviewableContentIdentifiedEventArgs}
DEFINE_IID!(IID_IWebViewControlWebResourceRequestedEventArgs, 1154896461, 21924, 19851, 137, 28, 147, 29, 142, 37, 212, 46);
RT_INTERFACE!{interface IWebViewControlWebResourceRequestedEventArgs(IWebViewControlWebResourceRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlWebResourceRequestedEventArgs] {
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT,
    fn get_Request(&self, out: *mut *mut super::http::HttpRequestMessage) -> HRESULT,
    fn put_Response(&self, value: *mut super::http::HttpResponseMessage) -> HRESULT,
    fn get_Response(&self, out: *mut *mut super::http::HttpResponseMessage) -> HRESULT
}}
impl ComPtr<IWebViewControlWebResourceRequestedEventArgs> {
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetDeferral)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_request(&self) -> Result<Option<ComPtr<super::http::HttpRequestMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Request)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_response(&self, value: &ComPtr<super::http::HttpResponseMessage>) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Response)(self.deref() as *const _ as *mut _, value.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_response(&self) -> Result<Option<ComPtr<super::http::HttpResponseMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Response)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlWebResourceRequestedEventArgs: IWebViewControlWebResourceRequestedEventArgs}
pub mod interop { // Windows.Web.UI.Interop
use crate::prelude::*;
RT_CLASS!{class WebViewControl: super::IWebViewControl}
DEFINE_IID!(IID_IWebViewControlAcceleratorKeyPressedEventArgs, 2007147838, 31860, 17277, 162, 144, 58, 192, 216, 205, 86, 85);
RT_INTERFACE!{interface IWebViewControlAcceleratorKeyPressedEventArgs(IWebViewControlAcceleratorKeyPressedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlAcceleratorKeyPressedEventArgs] {
    #[cfg(not(feature="windows-ui"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_EventType(&self, out: *mut crate::windows::ui::core::CoreAcceleratorKeyEventType) -> HRESULT,
    #[cfg(not(feature="windows-system"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-system")] fn get_VirtualKey(&self, out: *mut crate::windows::system::VirtualKey) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_KeyStatus(&self, out: *mut crate::windows::ui::core::CorePhysicalKeyStatus) -> HRESULT,
    fn get_RoutingStage(&self, out: *mut WebViewControlAcceleratorKeyRoutingStage) -> HRESULT,
    fn get_Handled(&self, out: *mut bool) -> HRESULT,
    fn put_Handled(&self, value: bool) -> HRESULT
}}
impl ComPtr<IWebViewControlAcceleratorKeyPressedEventArgs> {
    #[cfg(feature="windows-ui")] #[inline] pub fn get_event_type(&self) -> Result<crate::windows::ui::core::CoreAcceleratorKeyEventType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_EventType)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_virtual_key(&self) -> Result<crate::windows::system::VirtualKey> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_VirtualKey)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_key_status(&self) -> Result<crate::windows::ui::core::CorePhysicalKeyStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_KeyStatus)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_routing_stage(&self) -> Result<WebViewControlAcceleratorKeyRoutingStage> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_RoutingStage)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_handled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Handled)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_handled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Handled)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlAcceleratorKeyPressedEventArgs: IWebViewControlAcceleratorKeyPressedEventArgs}
RT_ENUM! { enum WebViewControlAcceleratorKeyRoutingStage: i32 {
    Tunneling = 0, Bubbling = 1,
}}
RT_ENUM! { enum WebViewControlMoveFocusReason: i32 {
    Programmatic = 0, Next = 1, Previous = 2,
}}
DEFINE_IID!(IID_IWebViewControlMoveFocusRequestedEventArgs, 1797927949, 19408, 16478, 183, 193, 30, 114, 164, 146, 244, 70);
RT_INTERFACE!{interface IWebViewControlMoveFocusRequestedEventArgs(IWebViewControlMoveFocusRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlMoveFocusRequestedEventArgs] {
    fn get_Reason(&self, out: *mut WebViewControlMoveFocusReason) -> HRESULT
}}
impl ComPtr<IWebViewControlMoveFocusRequestedEventArgs> {
    #[inline] pub fn get_reason(&self) -> Result<WebViewControlMoveFocusReason> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Reason)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlMoveFocusRequestedEventArgs: IWebViewControlMoveFocusRequestedEventArgs}
DEFINE_IID!(IID_IWebViewControlProcess, 46605292, 39126, 16970, 182, 62, 198, 19, 108, 54, 160, 242);
RT_INTERFACE!{interface IWebViewControlProcess(IWebViewControlProcessVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlProcess] {
    fn get_ProcessId(&self, out: *mut u32) -> HRESULT,
    fn get_EnterpriseId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsPrivateNetworkClientServerCapabilityEnabled(&self, out: *mut bool) -> HRESULT,
    fn CreateWebViewControlAsync(&self, hostWindowHandle: i64, bounds: foundation::Rect, out: *mut *mut foundation::IAsyncOperation<WebViewControl>) -> HRESULT,
    fn GetWebViewControls(&self, out: *mut *mut foundation::collections::IVectorView<WebViewControl>) -> HRESULT,
    fn Terminate(&self) -> HRESULT,
    fn add_ProcessExited(&self, handler: *mut foundation::TypedEventHandler<WebViewControlProcess, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ProcessExited(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<IWebViewControlProcess> {
    #[inline] pub fn get_process_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_ProcessId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_enterprise_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_EnterpriseId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_private_network_client_server_capability_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsPrivateNetworkClientServerCapabilityEnabled)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn create_web_view_control_async(&self, hostWindowHandle: i64, bounds: foundation::Rect) -> Result<ComPtr<foundation::IAsyncOperation<WebViewControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateWebViewControlAsync)(self.deref() as *const _ as *mut _, hostWindowHandle, bounds, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_web_view_controls(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<WebViewControl>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).GetWebViewControls)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn terminate(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Terminate)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_process_exited(&self, handler: &ComPtr<foundation::TypedEventHandler<WebViewControlProcess, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_ProcessExited)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_process_exited(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_ProcessExited)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlProcess: IWebViewControlProcess}
impl RtActivatable<IWebViewControlProcessFactory> for WebViewControlProcess {}
impl RtActivatable<IActivationFactory> for WebViewControlProcess {}
impl WebViewControlProcess {
    #[inline] pub fn create_with_options(processOptions: &ComPtr<WebViewControlProcessOptions>) -> Result<ComPtr<WebViewControlProcess>> {
        <Self as RtActivatable<IWebViewControlProcessFactory>>::get_activation_factory().create_with_options(processOptions)
    }
}
DEFINE_CLSID!(WebViewControlProcess(&[87,105,110,100,111,119,115,46,87,101,98,46,85,73,46,73,110,116,101,114,111,112,46,87,101,98,86,105,101,119,67,111,110,116,114,111,108,80,114,111,99,101,115,115,0]) [CLSID_WebViewControlProcess]);
RT_ENUM! { enum WebViewControlProcessCapabilityState: i32 {
    Default = 0, Disabled = 1, Enabled = 2,
}}
DEFINE_IID!(IID_IWebViewControlProcessFactory, 1203133689, 41682, 17724, 176, 151, 246, 119, 157, 75, 142, 2);
RT_INTERFACE!{static interface IWebViewControlProcessFactory(IWebViewControlProcessFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlProcessFactory] {
    fn CreateWithOptions(&self, processOptions: *mut WebViewControlProcessOptions, out: *mut *mut WebViewControlProcess) -> HRESULT
}}
impl ComPtr<IWebViewControlProcessFactory> {
    #[inline] pub fn create_with_options(&self, processOptions: &ComPtr<WebViewControlProcessOptions>) -> Result<ComPtr<WebViewControlProcess>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).CreateWithOptions)(self.deref() as *const _ as *mut _, processOptions.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebViewControlProcessOptions, 483029671, 15318, 18470, 130, 97, 108, 129, 137, 80, 93, 137);
RT_INTERFACE!{interface IWebViewControlProcessOptions(IWebViewControlProcessOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlProcessOptions] {
    fn put_EnterpriseId(&self, value: HSTRING) -> HRESULT,
    fn get_EnterpriseId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PrivateNetworkClientServerCapability(&self, value: WebViewControlProcessCapabilityState) -> HRESULT,
    fn get_PrivateNetworkClientServerCapability(&self, out: *mut WebViewControlProcessCapabilityState) -> HRESULT
}}
impl ComPtr<IWebViewControlProcessOptions> {
    #[inline] pub fn set_enterprise_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_EnterpriseId)(self.deref() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_enterprise_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_EnterpriseId)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_private_network_client_server_capability(&self, value: WebViewControlProcessCapabilityState) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_PrivateNetworkClientServerCapability)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_private_network_client_server_capability(&self) -> Result<WebViewControlProcessCapabilityState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_PrivateNetworkClientServerCapability)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WebViewControlProcessOptions: IWebViewControlProcessOptions}
impl RtActivatable<IActivationFactory> for WebViewControlProcessOptions {}
DEFINE_CLSID!(WebViewControlProcessOptions(&[87,105,110,100,111,119,115,46,87,101,98,46,85,73,46,73,110,116,101,114,111,112,46,87,101,98,86,105,101,119,67,111,110,116,114,111,108,80,114,111,99,101,115,115,79,112,116,105,111,110,115,0]) [CLSID_WebViewControlProcessOptions]);
DEFINE_IID!(IID_IWebViewControlSite, 322914246, 4828, 18584, 189, 71, 4, 150, 125, 230, 72, 186);
RT_INTERFACE!{interface IWebViewControlSite(IWebViewControlSiteVtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlSite] {
    fn get_Process(&self, out: *mut *mut WebViewControlProcess) -> HRESULT,
    fn put_Scale(&self, value: f64) -> HRESULT,
    fn get_Scale(&self, out: *mut f64) -> HRESULT,
    fn put_Bounds(&self, value: foundation::Rect) -> HRESULT,
    fn get_Bounds(&self, out: *mut foundation::Rect) -> HRESULT,
    fn put_IsVisible(&self, value: bool) -> HRESULT,
    fn get_IsVisible(&self, out: *mut bool) -> HRESULT,
    fn Close(&self) -> HRESULT,
    fn MoveFocus(&self, reason: WebViewControlMoveFocusReason) -> HRESULT,
    fn add_MoveFocusRequested(&self, handler: *mut foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_MoveFocusRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_AcceleratorKeyPressed(&self, handler: *mut foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AcceleratorKeyPressed(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<IWebViewControlSite> {
    #[inline] pub fn get_process(&self) -> Result<Option<ComPtr<WebViewControlProcess>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.deref().lpVtbl).get_Process)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_scale(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Scale)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_scale(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Scale)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_bounds(&self, value: foundation::Rect) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_Bounds)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bounds(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_Bounds)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_visible(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).put_IsVisible)(self.deref() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_visible(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).get_IsVisible)(self.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn close(&self) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).Close)(self.deref() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn move_focus(&self, reason: WebViewControlMoveFocusReason) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).MoveFocus)(self.deref() as *const _ as *mut _, reason);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_move_focus_requested(&self, handler: &ComPtr<foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_MoveFocusRequested)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_move_focus_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_MoveFocusRequested)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_accelerator_key_pressed(&self, handler: &ComPtr<foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_AcceleratorKeyPressed)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_accelerator_key_pressed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_AcceleratorKeyPressed)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebViewControlSite2, 3510316607, 18670, 18224, 130, 67, 210, 237, 12, 5, 96, 106);
RT_INTERFACE!{interface IWebViewControlSite2(IWebViewControlSite2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebViewControlSite2] {
    fn add_GotFocus(&self, handler: *mut foundation::TypedEventHandler<WebViewControl, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GotFocus(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_LostFocus(&self, handler: *mut foundation::TypedEventHandler<WebViewControl, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LostFocus(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<IWebViewControlSite2> {
    #[inline] pub fn add_got_focus(&self, handler: &ComPtr<foundation::TypedEventHandler<WebViewControl, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_GotFocus)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_got_focus(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_GotFocus)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_lost_focus(&self, handler: &ComPtr<foundation::TypedEventHandler<WebViewControl, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.deref().lpVtbl).add_LostFocus)(self.deref() as *const _ as *mut _, handler.deref() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_lost_focus(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.deref().lpVtbl).remove_LostFocus)(self.deref() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
} // Windows.Web.UI.Interop
} // Windows.Web.UI
